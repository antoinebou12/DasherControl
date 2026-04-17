use bollard::container::{
    Config, CreateContainerOptions, InspectContainerOptions, ListContainersOptions, LogsOptions,
};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use bytes::Bytes;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use tokio::io::AsyncWriteExt;

const COMPOSE_PROJECT_LABEL: &str = "com.docker.compose.project";
const COMPOSE_SERVICE_LABEL: &str = "com.docker.compose.service";

/// Hard cap for streamed container export size (bytes).
pub const MAX_CONTAINER_EXPORT_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerListItem {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: Option<String>,
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compose_project: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compose_service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Value>,
}

fn connect() -> Result<Docker, bollard::errors::Error> {
    Docker::connect_with_socket_defaults()
}

pub fn get_docker_interface() -> &'static Mutex<DockerInterface> {
    static DOCKER: OnceLock<Mutex<DockerInterface>> = OnceLock::new();
    DOCKER.get_or_init(|| {
        Mutex::new(DockerInterface::new().unwrap_or_else(|e| {
            tracing::warn!(error = %e, "Docker connect failed; continuing without Docker");
            DockerInterface::disconnected()
        }))
    })
}

/// Clone the connected client (if any) so async handlers do not hold `MutexGuard` across `.await`.
pub fn docker_client() -> Option<Docker> {
    get_docker_interface()
        .lock()
        .ok()
        .and_then(|g| g.docker.clone())
}

pub struct DockerInterface {
    pub docker: Option<Docker>,
}

impl DockerInterface {
    fn new() -> Result<Self, bollard::errors::Error> {
        Ok(Self {
            docker: Some(connect()?),
        })
    }

    fn disconnected() -> Self {
        Self { docker: None }
    }

    pub fn init(&mut self) {}
}

pub async fn list_containers(client: &Docker) -> Vec<DockerContainerListItem> {
    let opts = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };
    match client.list_containers(Some(opts)).await {
        Ok(list) => list
            .into_iter()
            .map(|c| {
                let labels_map = c.labels.clone().unwrap_or_default();
                let compose_project = labels_map.get(COMPOSE_PROJECT_LABEL).cloned();
                let compose_service = labels_map.get(COMPOSE_SERVICE_LABEL).cloned();
                let labels = c
                    .labels
                    .as_ref()
                    .and_then(|m| serde_json::to_value(m).ok());
                DockerContainerListItem {
                    id: c.id.unwrap_or_default(),
                    names: c.names.unwrap_or_default(),
                    image: c.image.unwrap_or_default(),
                    state: c.state,
                    status: c.status,
                    compose_project,
                    compose_service,
                    labels,
                }
            })
            .collect(),
        Err(e) => {
            tracing::warn!(error = %e, "list_containers failed");
            vec![]
        }
    }
}

pub async fn check_image_exist(client: &Docker, image: &str) -> bool {
    match client.list_images::<String>(None).await {
        Ok(images) => images
            .iter()
            .any(|img| img.repo_tags.iter().any(|t| t.as_str() == image)),
        Err(e) => {
            tracing::warn!(error = %e, image, "list_images failed");
            false
        }
    }
}

pub async fn pull_image(client: &Docker, image: &str) {
    let opts = CreateImageOptions {
        from_image: image.to_string(),
        ..Default::default()
    };
    let mut stream = client.create_image(Some(opts), None, None);
    while let Some(res) = stream.next().await {
        match res {
            Ok(output) => tracing::debug!(?output, "create_image progress"),
            Err(e) => tracing::warn!(error = %e, image, "pull_image stream error"),
        }
    }
}

pub async fn create_container(client: &Docker, image: &str) -> Result<String, String> {
    let config = Config::<String> {
        image: Some(image.to_string()),
        ..Default::default()
    };
    let res = client
        .create_container(None::<CreateContainerOptions<String>>, config)
        .await
        .map_err(|e| e.to_string())?;
    Ok(res.id)
}

pub async fn restart_container(client: &Docker, id: &str) -> Result<String, String> {
    client
        .restart_container(id, None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(id.to_string())
}

pub async fn stop_container(client: &Docker, id: &str) -> Result<String, String> {
    client
        .stop_container(id, None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(id.to_string())
}

pub async fn log_container(client: &Docker, id: String) {
    let _stream = client.logs(
        &id,
        Some(LogsOptions::<String> {
            stdout: true,
            stderr: true,
            ..Default::default()
        }),
    );
}

/// Streams `GET /containers/{id}/export` into `dest` (root filesystem tar).
pub async fn export_container_to_file(
    client: &Docker,
    container_id: &str,
    dest: &Path,
) -> Result<u64, String> {
    let mut file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| e.to_string())?;
    let mut stream = client.export_container(container_id);
    let mut written: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        let n = chunk.len() as u64;
        written = written.checked_add(n).ok_or_else(|| "export size overflow".to_string())?;
        if written > MAX_CONTAINER_EXPORT_BYTES {
            return Err(format!(
                "export exceeds max size ({} MiB)",
                MAX_CONTAINER_EXPORT_BYTES / (1024 * 1024)
            ));
        }
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
    }
    Ok(written)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerExportManifest {
    pub schema_version: u32,
    pub export_type: String,
    pub container_id: String,
    pub container_name: String,
    pub image: String,
    pub exported_at_rfc3339: String,
}

pub async fn build_export_manifest(
    client: &Docker,
    container_id: &str,
) -> Result<DockerExportManifest, String> {
    let ins = client
        .inspect_container(container_id, None::<InspectContainerOptions>)
        .await
        .map_err(|e| e.to_string())?;
    let id = ins.id.unwrap_or_else(|| container_id.to_string());
    let container_name = ins
        .name
        .unwrap_or_default()
        .trim_start_matches('/')
        .to_string();
    let image = ins
        .config
        .as_ref()
        .and_then(|c| c.image.clone())
        .unwrap_or_default();
    Ok(DockerExportManifest {
        schema_version: 1,
        export_type: "container_filesystem".to_string(),
        container_id: id,
        container_name,
        image,
        exported_at_rfc3339: chrono::Utc::now().to_rfc3339(),
    })
}

/// Import a root filesystem tar produced by [`export_container_to_file`] (`docker import` API).
pub async fn import_filesystem_tar(
    client: &Docker,
    tar: Bytes,
    repo: &str,
    tag: &str,
) -> Result<String, String> {
    let opts = CreateImageOptions::<String> {
        from_src: "-".into(),
        repo: repo.to_string(),
        tag: tag.to_string(),
        ..Default::default()
    };
    let mut stream = client.create_image(Some(opts), Some(tar), None);
    while let Some(item) = stream.next().await {
        let info = item.map_err(|e| e.to_string())?;
        if let Some(err) = info.error {
            return Err(err);
        }
    }
    Ok(format!("{repo}:{tag}"))
}
