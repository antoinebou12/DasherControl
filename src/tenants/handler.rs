
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

#[post("/tenants/create", format="json", data="<create_info>")]
fn create(conn:, create_info: Json<CreateInfo>) -> Json<i32> {
    let user: User = User {name: create_info.name.clone(), email: create_info.email.clone(), age: create_info.age};
    let user_entity: UserEntity = diesel::insert_into(users::table)
        .values(user)
        .get_result(&connection).expect("Error saving user");

    let password_hash = hash_password(&create_info.password);
    let auth_info: AuthInfo = AuthInfo {user_id: user_entity.id, password_hash: password_hash};
    let auth_info_entity: AuthInfoEntity = diesel::insert_into(auth_infos::table)
        .values(auth_info)
        .get_result(&connection).expect("Error saving auth info");
    Json(user_entity.id)
}