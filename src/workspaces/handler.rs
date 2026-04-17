use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::db::DbConn;
use crate::tenants::token::Claims;
use crate::workspaces::model::{
    Applet, NewApplet, NewWorkspace, NewWorkspaceWithApplets, Workspace, WorkspaceNoIDWithApplets,
    WorkspaceNoTenantWithApplets, WorkspaceWithApplets,
};

#[get("/api/<workspace_id>")]
pub async fn get_applets_in_workspace(
    conn: DbConn,
    workspace_id: i32,
    token: Claims,
) -> Result<Json<Vec<Applet>>, Status> {
    let workspace = match Workspace::get(&conn, &workspace_id) {
        Ok(workspace) => workspace,
        Err(_) => return Err(Status::Conflict),
    };
    if token.sub == workspace.tenant_id {
        Applet::all_applets_by_workspace(&conn, workspace_id)
            .map_err(error_status)
            .map(Json)
    } else {
        Err(Status::Conflict)
    }
}

#[get("/api/list")]
pub async fn get_workspaces(conn: DbConn, token: Claims) -> Result<Json<Vec<Workspace>>, Status> {
    Workspace::all_for_tenant(&conn, &token.sub)
        .map_err(error_status)
        .map(Json)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[post("/api/new", format = "json", data = "<workspace>")]
pub async fn create_workspace(
    conn: DbConn,
    workspace: Json<NewWorkspace>,
) -> Result<status::Accepted<String>, Status> {
    let new_workspace = workspace.into_inner();
    match Workspace::create(&conn, new_workspace) {
        Ok(_) => Ok(status::Accepted("workspace created".to_string())),
        Err(_) => Err(Status::Conflict),
    }
}

#[post("/api/update", format = "json", data = "<workspace>")]
pub async fn update_workspace(
    conn: DbConn,
    workspace: Json<WorkspaceNoTenantWithApplets>,
    token: Claims,
) -> Result<Json<String>, Status> {
    let workspace = workspace.into_inner();
    let existing = Workspace::get(&conn, &workspace.id).map_err(error_status)?;
    let update_workspace = WorkspaceWithApplets {
        id: workspace.id,
        display_order: workspace.display_order,
        name: workspace.name,
        tenant_id: token.get_tenant_id(),
        workspace_icon: workspace
            .workspace_icon
            .clone()
            .unwrap_or_else(|| existing.workspace_icon.clone()),
        theme_preset: workspace
            .theme_preset
            .clone()
            .unwrap_or_else(|| existing.theme_preset.clone()),
        bg_color: workspace.bg_color.clone().or(existing.bg_color.clone()),
        fg_color: workspace.fg_color.clone().or(existing.fg_color.clone()),
        accent_color: workspace
            .accent_color
            .clone()
            .or(existing.accent_color.clone()),
        panel_color: workspace.panel_color.clone().or(existing.panel_color.clone()),
        background_image_url: workspace
            .background_image_url
            .clone()
            .or(existing.background_image_url.clone()),
        applets: workspace.applets,
    };
    match Workspace::update_with_applets(&conn, update_workspace) {
        Ok(_) => Ok(Json("workspace updated".to_string())),
        Err(_) => Err(Status::Conflict),
    }
}

#[post("/api/applets/create", format = "json", data = "<applet>")]
pub async fn create_applet(conn: DbConn, applet: Json<NewApplet>) -> Result<status::Accepted<String>, String> {
    let new_applet = applet.into_inner();
    match Applet::create(&conn, new_applet) {
        Ok(_) => Ok(status::Accepted("applets created".to_string())),
        Err(e) => Err(e.to_string()),
    }
}

#[post("/api/create", format = "json", data = "<workspace>")]
pub async fn create_workspace_with_applets(
    conn: DbConn,
    workspace: Json<WorkspaceNoIDWithApplets>,
    token: Claims,
) -> Result<Json<String>, Status> {
    let workspace_no_id = workspace.into_inner();
    let new_workspace = NewWorkspaceWithApplets {
        display_order: workspace_no_id.display_order,
        name: workspace_no_id.name,
        tenant_id: token.get_tenant_id(),
        workspace_icon: workspace_no_id.workspace_icon,
        theme_preset: workspace_no_id.theme_preset,
        bg_color: workspace_no_id.bg_color,
        fg_color: workspace_no_id.fg_color,
        accent_color: workspace_no_id.accent_color,
        panel_color: workspace_no_id.panel_color,
        background_image_url: workspace_no_id.background_image_url,
        applets: workspace_no_id.applets,
    };
    match Workspace::create_with_applets(&conn, new_workspace) {
        Ok(_) => Ok(Json("workspace with applets created".to_string())),
        Err(_) => Err(Status::Conflict),
    }
}

#[post("/api/delete/<workspace_id>")]
pub async fn delete_workspace_with_applets(
    conn: DbConn,
    workspace_id: i32,
    _token: Claims,
) -> Result<Json<String>, Status> {
    Workspace::delete_with_applets(&conn, &workspace_id);
    Ok(Json("workspace with applets deleted".to_string()))
}
