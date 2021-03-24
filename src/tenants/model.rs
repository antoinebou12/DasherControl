use crate::tenants::schema::*;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "tenants"]
pub struct Tenant {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
}

// #[derive(Insertable, Debug)]
// pub struct TenantInstance<'a> {
//     pub email: &'a str,
//     pub name: &'a str,
//     pub username: &'a str,
//     pub password: &'a str,
//     pub role: &'a str,
//     pub created_at: chrono::NaiveDateTime,
// }
