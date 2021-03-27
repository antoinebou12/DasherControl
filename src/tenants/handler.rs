use std::fmt;

use diesel::prelude::*;
use diesel::result::Error;

use rocket::http::Status;
use rocket::response::status;
use rocket::http::hyper::header::Encoding;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;

use rocket_contrib::json::Json;

use crate::db::DbConn;
use crate::tenants::model::Tenant;
use crate::tenants::model::RegisterTenant;
use crate::tenants::model::AuthTenant;
use crate::tenants::schema::tenants;
use crate::tenants::helper::all;
use crate::tenants::error::MyStoreError;
use crate::tenants::jwt::*;

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


#[post("/json/create", format="application/json", data = "<tenant>")]
pub fn create_tenant(conn: DbConn, tenant: Json<RegisterTenant>) -> () {
    let register_tenant = tenant.into_inner();
    Tenant::create(register_tenant, &conn);
}


#[post("/json/login", format="application/json", data = "<tenant_login>")]
pub fn login(conn: DbConn, tenant_login: Json<AuthTenant>) -> status::Accepted<String> {

    let tenant = tenant_login
        .login(&conn)
        .map_err(|e| {
            match e {
                MyStoreError::DBError(diesel::result::Error::NotFound) =>
                    status::Accepted(Some(e.to_string())),
                _ =>
                    status::Accepted(Some(e.to_string()))
            }
        })?;

    // This is the jwt token we will send in a cookie.
    let token = create_token(tenant.id, &tenant.email, &tenant.name);

    status::Accepted(Some(token.to_string()))

    // Finally our response will have a csrf token for security. 
    // hex::encode(generator.generate())
}


// #[post("/json/logout")]
// pub fn logout() -> () {
    
// }

