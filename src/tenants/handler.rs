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


#[post("/json/create", format="application/json", data = "<tenant>")]
pub fn create_tenant(conn: DbConn, tenant: Json<RegisterTenant>) -> () {
    let register_tenant = tenant.into_inner();
    Tenant::create(register_tenant, &conn);
}


// #[post("/json/login", format="application/json", data = "<tenant>")]
// pub fn login(
//     conn: DbConn,
//     tenant: Json<AuthUser>, 
//     id: Identity, 
//     generator: Json<CsrfTokenGenerator>) -> () {

//     let user = auth_user
//         .login(&conn)
//         .map_err(|e| {
//             match e {
//                 MyStoreError::DBError(diesel::result::Error::NotFound) =>
//                     printfn(e.to_string()),
//                 _ =>
//                     printfn(e.to_string())
//             }
//         })?;

//     // This is the jwt token we will send in a cookie.
//     let token = create_token(&user.email, &user.name)?;

//     id.remember(token);

//     // Finally our response will have a csrf token for security. 
//     let response =
//         HttpResponse::Ok()
//         .header("X-CSRF-TOKEN", hex::encode(generator.generate()))
//         .json(user);
//     Ok(response);
// }


// #[post("/json/logout")]
// pub fn logout(id: Identity) -> () {
//     id.forget();
// }

