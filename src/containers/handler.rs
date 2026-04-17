use std::io::{Cursor, Read, Write};

use bytes::Bytes;
use diesel::result::Error;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use tokio::task::spawn_blocking;

use crate::containers::docker::{
    build_export_manifest, check_image_exist, create_container, docker_client,
    export_container_to_file, import_filesystem_tar, list_containers, pull_image,
    restart_container, stop_container, DockerContainerListItem, DockerExportManifest,
    MAX_CONTAINER_EXPORT_BYTES,
};
use crate::containers::model::{Container, DockerHostContainer, NewContainer};
use crate::db::DbConn;
use crate::tenants::token::Claims;

/// Max uploaded zip size (bytes); should exceed largest plausible export zip.
const MAX_DOCKER_IMPORT_ZIP_BYTES: u64 = MAX_CONTAINER_EXPORT_BYTES + 64 * 1024 * 1024;

#[derive(Serialize)]
pub struct DockerImportResponse {
    pub repository: String,
    pub tag: String,
    pub image_ref: String,
}

#[derive(FromForm)]
pub struct DockerImportForm<'r> {
    pub file: TempFile<'r>,
}

fn sanitize_image_part(input: Option<String>, default: &'static str) -> String {
    let s = input.as_deref().unwrap_or(default);
    let out: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/'))
        .collect();
    if out.is_empty() {
        default.to_string()
    } else {
        out
    }
}

fn extract_docker_import_zip(zip_bytes: &[u8]) -> Result<(DockerExportManifest, Vec<u8>), String> {
    let reader = Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;
    let mut manifest_bytes: Option<Vec<u8>> = None;
    let mut rootfs_bytes: Option<Vec<u8>> = None;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let raw = file.name().to_string();
        if raw.contains("..") {
            return Err("illegal path in zip".into());
        }
        let name = raw.trim_start_matches("./");
        if name.contains('/') {
            return Err("nested paths in zip are not allowed".into());
        }
        let uncompressed = file.size();
        if uncompressed > MAX_CONTAINER_EXPORT_BYTES {
            return Err(format!(
                "rootfs.tar exceeds max size ({} MiB)",
                MAX_CONTAINER_EXPORT_BYTES / (1024 * 1024)
            ));
        }
        match name {
            "manifest.json" => {
                const MAX_MANIFEST: u64 = 256 * 1024;
                if uncompressed > MAX_MANIFEST {
                    return Err("manifest.json is too large".into());
                }
                let mut v = Vec::new();
                file.read_to_end(&mut v).map_err(|e| e.to_string())?;
                manifest_bytes = Some(v);
            }
            "rootfs.tar" => {
                let mut v = Vec::new();
                file.read_to_end(&mut v).map_err(|e| e.to_string())?;
                rootfs_bytes = Some(v);
            }
            other => return Err(format!("unexpected zip entry: {other}")),
        }
    }
    let manifest_raw = manifest_bytes.ok_or_else(|| "missing manifest.json".to_string())?;
    let rootfs = rootfs_bytes.ok_or_else(|| "missing rootfs.tar".to_string())?;
    let manifest: DockerExportManifest =
        serde_json::from_slice(&manifest_raw).map_err(|e| e.to_string())?;
    if manifest.schema_version != 1 {
        return Err("unsupported manifest schema_version".into());
    }
    if manifest.export_type != "container_filesystem" {
        return Err("unsupported export_type in manifest".into());
    }
    Ok((manifest, rootfs))
}

#[get("/api/list")]
pub async fn get_containers(conn: DbConn) -> Result<Json<Vec<Container>>, Status> {
    Container::all(&conn)
        .map_err(error_status)
        .map(Json)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[put("/api/new", format = "json", data = "<container>")]
pub async fn create_container_route(
    conn: DbConn,
    container: Json<NewContainer>,
) -> Result<Json<String>, Status> {
    let new_container = container.into_inner();
    match Container::create(&conn, new_container) {
        Ok(_) => Ok(Json("container created".to_string())),
        Err(_) => Err(Status::Conflict),
    }
}

#[get("/api/docker_list")]
pub async fn get_docker_containers(_conn: DbConn) -> Json<Vec<DockerContainerListItem>> {
    let out = match docker_client() {
        Some(ref c) => list_containers(c).await,
        None => vec![],
    };
    Json(out)
}

/// Last snapshot written by [`post_docker_snapshot`](post_docker_snapshot) (compose labels, names, state).
#[get("/api/docker_snapshot")]
pub async fn get_docker_snapshot(conn: DbConn) -> Result<Json<Vec<DockerContainerListItem>>, Status> {
    DockerHostContainer::all(&conn)
        .map(|rows| Json(rows.into_iter().map(|r| r.to_list_item()).collect()))
        .map_err(error_status)
}

/// Refreshes the `docker_host_containers` table from the local Docker daemon (Admin only).
#[post("/api/docker_snapshot")]
pub async fn post_docker_snapshot(
    conn: DbConn,
    token: Claims,
) -> Result<Json<Vec<DockerContainerListItem>>, Status> {
    if !token.has_role("Admin") {
        return Err(Status::Unauthorized);
    }
    let items = match docker_client() {
        Some(ref c) => list_containers(c).await,
        None => return Err(Status::ServiceUnavailable),
    };
    DockerHostContainer::replace_snapshot(&conn, &items).map_err(error_status)?;
    DockerHostContainer::all(&conn)
        .map(|rows| Json(rows.into_iter().map(|r| r.to_list_item()).collect()))
        .map_err(error_status)
}

#[get("/api/create_container")]
pub async fn create_docker_containers(_conn: DbConn, token: Claims) -> Result<Json<String>, Status> {
    if token.has_role("Admin") {
        let Some(ref c) = docker_client() else {
            return Err(Status::ServiceUnavailable);
        };
        if !check_image_exist(c, "hello-world").await {
            pull_image(c, "hello-world").await;
        }
        let id = create_container(c, "hello-world")
            .await
            .map_err(|_| Status::Conflict)?;
        Ok(Json(id))
    } else {
        Err(Status::Unauthorized)
    }
}

#[post("/api/restart/<container_id>")]
pub async fn restart_docker_container(
    _conn: DbConn,
    container_id: String,
    token: Claims,
) -> Result<Json<String>, Status> {
    if token.has_role("Admin") {
        let Some(ref c) = docker_client() else {
            return Err(Status::ServiceUnavailable);
        };
        match restart_container(c, &container_id).await {
            Ok(container) => Ok(Json(format!("container {container} restarted"))),
            Err(_) => Err(Status::Conflict),
        }
    } else {
        Err(Status::Unauthorized)
    }
}

#[post("/api/stop/<container_id>")]
pub async fn stop_docker_container(
    _conn: DbConn,
    container_id: String,
    token: Claims,
) -> Result<Json<String>, Status> {
    if token.has_role("Admin") {
        let Some(ref c) = docker_client() else {
            return Err(Status::ServiceUnavailable);
        };
        match stop_container(c, &container_id).await {
            Ok(container) => Ok(Json(format!("container {container} stop"))),
            Err(_) => Err(Status::Conflict),
        }
    } else {
        Err(Status::Unauthorized)
    }
}

#[get("/api/docker_export_zip/<container_id>")]
pub async fn docker_export_zip(
    _conn: DbConn,
    container_id: String,
    token: Claims,
) -> Result<(ContentType, Vec<u8>), Status> {
    if !token.has_role("Admin") {
        return Err(Status::Unauthorized);
    }
    let Some(ref client) = docker_client() else {
        return Err(Status::ServiceUnavailable);
    };
    let manifest = build_export_manifest(client, &container_id)
        .await
        .map_err(|e| {
            eprintln!("docker_export_zip inspect: {e}");
            Status::Conflict
        })?;
    let manifest_json =
        serde_json::to_vec_pretty(&manifest).map_err(|_| Status::InternalServerError)?;
    let dir = tempfile::tempdir().map_err(|_| Status::InternalServerError)?;
    let tar_path = dir.path().join("rootfs.tar");
    export_container_to_file(client, &container_id, &tar_path)
        .await
        .map_err(|e| {
            eprintln!("docker_export_zip export: {e}");
            Status::Conflict
        })?;
    let tar_path_cl = tar_path.clone();
    let zip_bytes = spawn_blocking(move || -> Result<Vec<u8>, String> {
        let mut buf = Cursor::new(Vec::new());
        let mut zip = zip::ZipWriter::new(&mut buf);
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zip.start_file("manifest.json", opts)
            .map_err(|e| e.to_string())?;
        zip.write_all(&manifest_json)
            .map_err(|e| e.to_string())?;
        zip.start_file(
            "rootfs.tar",
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored),
        )
        .map_err(|e| e.to_string())?;
        let mut tar_file = std::fs::File::open(&tar_path_cl).map_err(|e| e.to_string())?;
        std::io::copy(&mut tar_file, &mut zip).map_err(|e| e.to_string())?;
        zip.finish().map_err(|e| e.to_string())?;
        Ok(buf.into_inner())
    })
    .await
    .map_err(|_| Status::InternalServerError)?
    .map_err(|_| Status::InternalServerError)?;
    if zip_bytes.len() as u64 > MAX_DOCKER_IMPORT_ZIP_BYTES {
        return Err(Status::PayloadTooLarge);
    }
    Ok((ContentType::new("application", "zip"), zip_bytes))
}

#[post("/api/docker_import_zip?<repo>&<tag>", data = "<form>")]
pub async fn post_docker_import_zip(
    _conn: DbConn,
    repo: Option<String>,
    tag: Option<String>,
    form: Form<DockerImportForm<'_>>,
    token: Claims,
) -> Result<Json<DockerImportResponse>, Status> {
    if !token.has_role("Admin") {
        return Err(Status::Unauthorized);
    }
    let Some(ref client) = docker_client() else {
        return Err(Status::ServiceUnavailable);
    };
    let form = form.into_inner();
    let path = form.file.path().ok_or(Status::BadRequest)?;
    let meta = tokio::fs::metadata(path)
        .await
        .map_err(|_| Status::InternalServerError)?;
    if meta.len() > MAX_DOCKER_IMPORT_ZIP_BYTES {
        return Err(Status::PayloadTooLarge);
    }
    let raw = tokio::fs::read(path)
        .await
        .map_err(|_| Status::InternalServerError)?;
    let (_manifest, rootfs_tar) = spawn_blocking(move || extract_docker_import_zip(&raw))
        .await
        .map_err(|_| Status::InternalServerError)?
        .map_err(|e| {
            eprintln!("docker_import_zip: {e}");
            Status::BadRequest
        })?;
    let mut repo_s = sanitize_image_part(repo, "dasher/import");
    let tag_s = sanitize_image_part(tag, "latest").replace('/', "-");
    if !repo_s.contains('/') {
        repo_s = format!("library/{repo_s}");
    }
    let image_ref = import_filesystem_tar(client, Bytes::from(rootfs_tar), &repo_s, &tag_s)
        .await
        .map_err(|e| {
            eprintln!("docker import: {e}");
            Status::Conflict
        })?;
    Ok(Json(DockerImportResponse {
        repository: repo_s,
        tag: tag_s,
        image_ref,
    }))
}
