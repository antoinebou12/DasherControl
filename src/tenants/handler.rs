
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

use diesel::result::Error;

use crate::db::DbConn;
use crate::tenants;
use crate::tenants::model::Tenant;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use std::env;

fn hash_password(password: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

#[get("/tenants/all")]
pub fn fetch_all_tenants(conn: DbConn) -> Json<Vec<Tenant>> {
    return Json(tenants.load::<Tenant>(&conn).expect("Error loading users"))
}

#[get("/tenants/<uid>")]
pub fn fetch_tenant(conn: DbConn, uid: i32) -> Option<Json<Tenant>> {
    let mut tenants_by_id: Vec<Tenant> = tenants.filter(id.eq(uid))
        .load::<Tenant>(&conn).expect("Error loading users");
    if tenants_by_id.len() == 0 {
        return None
    } else {
        let first_user = tenants_by_id.remove(0);
        return Some(Json(first_user))
    }
}

#[post("/tenants/create", format="application/json", data = "<user>")]
fn create_tenant(conn: DbConn, user: Json<User>) -> Json<i32> {
    let user_entity: Tenant = diesel::insert_into(tenants::table)
        .values(&*user)
        .get_result(&connection).expect("Error saving user");
    Json(user_entity.id)
}
