use crate::tenants::schema::tenants;


#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "tenants"]
pub struct Tenant {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: String,
}


#[derive(Insertable)]
#[table_name = "tenants"]
pub struct CreateTenant {
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable)]
pub struct AuthInfo {
    pub tenant_id: i32,
    pub password_hash: String,
}

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}


