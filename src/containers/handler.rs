use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use diesel::result::Error;
use crate::containers::docker::{get_docker_interface};
use crate::containers::model::{Container, NewContainer};
use crate::db::DbConn;
use crate::tenants::token::Claims;


#[get("/api/list")]
pub fn get_containers(conn: DbConn) -> Result<Json<Vec<Container>>, Status> {
    return Container::all(&conn)
        .map_err(|error| error_status(error))
        .map(|container| Json(container));
}


fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}


#[put("/api/new", format="application/json", data = "<container>")]
pub fn create_container(conn: DbConn, container: Json<NewContainer>) -> Result<Json<String>, Status> {
    let new_container = container.into_inner();
    let _container_create = match Container::create( &conn, new_container) {
        Ok(_container) => return Ok(Json("container created".to_string())),
        Err(_) => return Err(Status::Conflict)
    };
}

#[get("/api/docker_list")]
pub fn get_docker_containers(conn: DbConn) -> Json<Vec<shiplift::rep::Container>> {
    let docker = get_docker_interface().lock().unwrap();
    let containers = docker.get_containers();
    return Json(containers);
}

#[get("/api/create_container")]
pub fn create_docker_containers(conn: DbConn, token: Result<Claims, Status>) -> Result<Json<String>, Status> {
    let token = match token {
        Ok(token) => token,
        Err(e) => return Err(e)
    };
    return if token.has_role("Admin") {
        let docker = get_docker_interface().lock().unwrap();
        if !docker.check_image_exist("hello-world") {
            docker.pull_image("hello-world");
        }
        let id = match docker.create_docker_container("hello-world"){
            Ok(id) => id,
            Err(_e) => return Err(Status::Conflict)
        };
        return Ok(Json(id.to_string()));
    } else {
        Err(Status::Unauthorized)
    }
}


#[post("/api/restart/<container_id>")]
pub fn restart_docker_container(conn: DbConn, container_id: String, token: Result<Claims, Status>) -> Result<Json<String>, Status> {
    let token = match token {
        Ok(token) => token,
        Err(e) => return Err(e)
    };
    return if token.has_role("Admin") {
        let docker = get_docker_interface().lock().unwrap();
        let _container_restart = return match docker.restart_container(&*container_id) {
            Ok(container) => Ok(Json(format!("container {} restarted", container))),
            Err(_e) => Err(Status::Conflict)
        };
    } else {
        Err(Status::Unauthorized)
    }
}

#[post("/api/stop/<container_id>")]
pub fn stop_docker_container(conn: DbConn, container_id: String, token: Result<Claims, Status>) -> Result<Json<String>, Status> {
    let token = match token {
        Ok(token) => token,
        Err(e) => return Err(e)
    };
    return if token.has_role("Admin") {
        let docker = get_docker_interface().lock().unwrap();
        let _container_stop = return match docker.stop_container(&*container_id) {
            Ok(container) => Ok(
                Json(format!("container {} stop", container))),
            Err(_) => Err(Status::Conflict)
        };
    } else {
        Err(Status::Unauthorized)
    }
}