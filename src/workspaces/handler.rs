use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db::DbConn;
use crate::workspaces::model::{NewWorkspace, Workspace, NewApplet, Applet};
use diesel::result::Error;

#[get("/api/applets/<workspace_id>")]
pub fn get_applets_in_workspace(conn: DbConn, workspace_id: i32) -> Result<Json<Vec<Applet>>, Status> {
    return Applet::all_applets_by_workspace(workspace_id, &conn)
        .map_err(|error| error_status(error))
        .map(|applets| Json(applets));
}


fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/api/create", format="application/json", data = "<workspace>")]
pub fn create_workspace(conn: DbConn, workspace: Json<NewWorkspace>) -> Result<status::Accepted<String>, Status> {
    let new_workspace = workspace.into_inner();
    let workspace_create = match Workspace::create(new_workspace, &conn) {
        Ok(workspace) => return Ok(status::Accepted(Some("workspace created".to_string()))),
        Err(_) => return Err(Status::Conflict)
    };

}

#[post("/api/applets/create", format="application/json", data = "<applet>")]
pub fn create_applet(conn: DbConn, applet: Json<NewApplet>) -> Result<status::Accepted<String>, String> {
    let new_applet = applet.into_inner();
    let applet_create = match Applet::create(new_applet, &conn) {
        Ok(applet) => return Ok(status::Accepted(Some("applets created".to_string()))),
        Err(e) => return Err(format!("{:?}", e.to_string()))
    };
}