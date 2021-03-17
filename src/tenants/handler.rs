
use diesel::result::Error;

use crate::db::DbConn;
use crate::tenants;
use crate::tenants::Tenant;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use std::env;

pub fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

pub fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}


#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Tenant>>, Status> {
    tenants::repository::all(&connection)
        .map(|tenants| Json(tenants))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Tenant>, Status> {
    tenants::repository::get(id, &connection)
        .map(|tenant| Json(tenant))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<tenant>")]
pub fn post(tenant: Json<Tenant>, connection: DbConn) -> Result<status::Created<Json<Tenant>>, Status> {
    tenants::repository::insert(tenant.into_inner(), &connection)
        .map(|tenant| tenant_created(tenant))
        .map_err(|error| error_status(error))
}

fn tenant_created(tenant: Tenant) -> status::Created<Json<Tenant>> {
    status::Created(
        format!("{host}:{port}/tenant/{id}", host = host(), port = port(), id = tenant.id).to_string(),
        Some(Json(tenant)))
}

#[put("/<id>", format = "application/json", data = "<tenant>")]
pub fn put(id: i32, tenant: Json<Tenant>, connection: DbConn) -> Result<Json<Tenant>, Status> {
    tenants::repository::update(id, tenant.into_inner(), &connection)
        .map(|tenant| Json(tenant))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match tenants::repository::get(id, &connection) {
        Ok(_) => tenants::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}