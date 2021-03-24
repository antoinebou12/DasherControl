use diesel::prelude::*;
use diesel::result::Error;

use rocket::http::Status;
use rocket::response::status;

use rocket_contrib::json::Json;

use crate::db::DbConn;
use crate::tenants::model::Tenant;
use crate::tenants::schema::tenants;
use crate::tenants::helper::all;


#[get("/")]
pub fn all_tenants(conn: DbConn) -> Result<Json<Vec<Tenant>>, Status> {
    return all(&conn)
    .map(|tenants| Json(tenants));
    .map_err(|error| error_status(error))

}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}


// #[post("/create", format="application/json", data = "<tenant>")]
// pub fn create_tenant(conn: DbConn, tenant: Json<Tenant>) -> Json<String> {
//     let insert_tenant = tenant { ..user.into_inner() };
//     insert(&conn, insert_tenant);
//     return Json("true".to_string());
// }


