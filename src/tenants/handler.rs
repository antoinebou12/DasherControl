use crate::tenants::model::*;
use crate::tenants::schema::*;
use crate::tenants::helper::*;

use rocket_contrib::json::Json;

use crate::db::DbConn;

// #[get("/<uid>")]
// pub fn fetch_tenant(conn: DbConn, id: i32) -> Option<Json<Tenant>> {
//     return Json(Some(helper::get(&conn, id)))
// }

#[post("/create", format="application/json", data = "<tenant>")]
pub fn create_tenant(conn: DbConn, tenant: Json<Tenant>) -> Json<String> {
    let insert_tenant = tenant { ..user.into_inner() };
    insert(&conn, insert_tenant);
    return Json("true".to_string());
}


