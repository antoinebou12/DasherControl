use diesel::prelude::*;
use diesel::result::Error;

use rocket::http::Status;
use rocket::response::status;

use rocket_contrib::json::Json;

use crate::db::DbConn;
use crate::tenants::model::Tenant;
use crate::tenants::model::RegisterTenant;
use crate::tenants::schema::tenants;
use crate::tenants::helper::all;


#[get("/")]
pub fn all_tenants(conn: DbConn) -> Result<Json<Vec<Tenant>>, Status> {
    return all(&conn)
    .map_err(|error| error_status(error))
    .map(|tenants| Json(tenants));
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}


#[post("/create", format="application/json", data = "<tenant>")]
pub fn create_tenant(conn: DbConn, tenant: Json<RegisterTenant>) -> () {
    let register_tenant = tenant.into_inner();
    Tenant::create(register_tenant, &conn);
}


