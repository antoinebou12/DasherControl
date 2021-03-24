use crate::tenants::schema::*;


#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[table_name = "tenants"]
pub struct Tenant {
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub tenant_role: String,
}

#[derive(Queryable, Serialize)]
pub struct TenantInstance {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub tenant_roles: Vec<String>,
}

#[derive(Insertable)]
pub struct AuthInfo {
    pub tenant_id: i32,
    pub password_hash: String,
}

#[derive(Queryable)]
pub struct AuthInfoEntity {
    pub id: i32,
    pub tenant_id: i32,
    pub password_hash: String,
}

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword
}

struct AuthenticatedUser {
    user_id: i32
}
