#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::tenants;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "tenants"]
pub struct Tenant {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub role: String,
}