use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::db::DbConn;
use crate::tenants::helper::all;
use crate::tenants::jwt::*;
use crate::tenants::model::AuthTenant;
use crate::tenants::model::RegisterTenant;
use crate::tenants::model::Tenant;

#[get("/list")]
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


#[post("/json/create", format="application/json", data = "<tenant>")]
pub fn create_tenant(conn: DbConn, tenant: Json<RegisterTenant>) -> Result<status::Accepted<String>, Status> {
    let register_tenant = match tenant.into_inner().validates(&conn) {
        Ok(register_tenant) => register_tenant,
        Err(_) => return Err(Status::Conflict),
    };
    let tenant_create = match Tenant::create(register_tenant, &conn) {
        Ok(tenant) => return Ok(status::Accepted(Some("yee".to_string()))),
        Err(_) => return Err(Status::Conflict),
    };

}


#[post("/json/login", format="application/json", data = "<auth_tenant>")]
pub fn login(conn: DbConn, auth_tenant: Json<AuthTenant>) -> Result<status::Accepted<String>, Status> {

    let tenant = match auth_tenant.login(&conn) {
        Ok(tenant) => tenant,
        Err(_) => return Err(Status::Conflict),
    };
    // This is the jwt token we will send in a cookie.
    let token = create_token(tenant.id, &tenant.email, &tenant.name).unwrap();

    return Ok(status::Accepted(Some(format!("token: '{}'", token.to_string()))));

    // Finally our response will have a csrf token for security. 
    // hex::encode(generator.generate())
}



// #[post("/json/logout")]
// pub fn logout() -> () {
    
// }

