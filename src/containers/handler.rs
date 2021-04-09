use futures::executor;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use diesel::result::Error;
use crate::containers::docker::{DockerInterface, get_docker_interface};
use crate::containers::model::{Container, NewContainer};
use crate::db::DbConn;


#[get("/api/list")]
pub fn get_containers(conn: DbConn) -> Result<Json<Vec<Container>>, Status> {
    return Container::all(&conn)
        .map_err(|error| error_status(error))
        .map(|container| Json(container));
}

#[get("/api/running_list")]
pub fn get_real_containers(conn: DbConn) -> Json<Vec<shiplift::rep::Container>> {
    let docker = get_docker_interface().lock().unwrap();
    let containers = docker.get_containers();
    return Json(containers);
}

#[get("/api/create_container")]
pub fn create_real_containers(conn: DbConn) -> Result<status::Accepted<String>, Status> {
    let docker = get_docker_interface().lock().unwrap();
    if !docker.check_image_exist("hello-world") {
        docker.pull_image("hello-world");
    }
    let id = match docker.create_docker_container("hello-world"){
        Ok(id) => id,
        Err(e) => return Err(Status::Conflict)
    };
    return Ok(status::Accepted(Some(id.to_string())));
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