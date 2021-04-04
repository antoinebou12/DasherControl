use crate::db::DbConn;
use rocket_contrib::json::Json;
use crate::containers::model::{Container, NewContainer};
use rocket::http::Status;

use diesel::result::Error;
use rocket::response::status;
use crate::containers::docker::{containers_list, images_list};

#[get("/api/list")]
pub fn get_containers(conn: DbConn) -> Result<Json<Vec<Container>>, Status> {
    return Container::all(&conn)
        .map_err(|error| error_status(error))
        .map(|container| Json(container));
}

#[get("/api/real_list")]
pub fn get_real_containers(conn: DbConn) -> () {
    containers_list();
    images_list();
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[post("/api/new", format="application/json", data = "<container>")]
pub fn create_container(conn: DbConn, container: Json<NewContainer>) -> Result<status::Accepted<String>, Status> {
    let new_container = container.into_inner();
    let container_create = match Container::create( &conn, new_container) {
        Ok(container) => return Ok(status::Accepted(Some("container created".to_string()))),
        Err(_) => return Err(Status::Conflict)
    };

}