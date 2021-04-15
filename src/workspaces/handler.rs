use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db::DbConn;
use crate::workspaces::model::{NewWorkspace, Workspace, NewApplet, Applet, NewWorkspaceWithApplets};
use diesel::result::Error;
use crate::tenants::token::Claims;

#[get("/api/<workspace_id>")]
pub fn get_applets_in_workspace(conn: DbConn, workspace_id: i32) -> Result<Json<Vec<Applet>>, Status> {
    return Applet::all_applets_by_workspace(&conn, workspace_id)
        .map_err(|error| error_status(error))
        .map(|applets| Json(applets));
}

#[get("/api/list")]
pub fn get_workspaces(conn: DbConn, token: Result<Claims, Status>) -> Result<Json<Vec<Workspace>>, Status> {
    let token = match token {
        Ok(token) => token,
        Err(e) => return Err(e)
    };
    return Workspace::all_for_tenant(&conn, &token.sub)
        .map_err(|error| error_status(error))
        .map(|workspace| Json(workspace));
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/api/new", format="application/json", data = "<workspace>")]
pub fn create_workspace(conn: DbConn, workspace: Json<NewWorkspace>) -> Result<status::Accepted<String>, Status> {
    let new_workspace = workspace.into_inner();
    let _workspace_create = match Workspace::create( &conn, new_workspace,) {
        Ok(_workspace) => return Ok(status::Accepted(Some("workspace created".to_string()))),
        Err(_) => return Err(Status::Conflict)
    };

}

#[post("/api/applets/create", format="application/json", data = "<applet>")]
pub fn create_applet(conn: DbConn, applet: Json<NewApplet>) -> Result<status::Accepted<String>, String> {
    let new_applet = applet.into_inner();
    let _applet_create = match Applet::create(&conn, new_applet) {
        Ok(_applet) => return Ok(status::Accepted(Some("applets created".to_string()))),
        Err(e) => return Err(format!("{:?}", e.to_string()))
    };
}

#[post("/api/create", format="application/json", data = "<workspace>")]
pub fn create_workspace_with_applets(conn: DbConn, workspace: Json<NewWorkspaceWithApplets>, ) -> Result<status::Accepted<String>, Status> {
    let new_workspace = workspace.into_inner();
    let _workspace_create = match Workspace::create_with_applets(&conn, new_workspace) {
        Ok(_workspace) => return Ok(status::Accepted(Some("workspace with applets created".to_string()))),
        Err(_) => return Err(Status::Conflict)
    };
}