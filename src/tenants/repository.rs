#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::schema::tenants;
use crate::tenants::Tenant;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Tenant>> {
    tenants::table.load::<Tenant>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Tenant> {
    tenants::table.find(id).get_result::<Tenant>(connection)
}

pub fn insert(tenant: Tenant, connection: &PgConnection) -> QueryResult<Tenant> {
    diesel::insert_into(tenants::table)
        .values(&InsertableTenant::from_tenant(tenant))
        .get_result(connection)
}

pub fn update(id: i32, tenant: Tenant, connection: &PgConnection) -> QueryResult<Tenant> {
    diesel::update(tenants::table.find(id))
        .set(&tenant)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(tenants::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "tenants"]
struct InsertableTenant {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub role: String,
}

impl InsertableTenant {

    fn from_tenant(tenant: Tenant) -> InsertableTenant {
        InsertableTenant {
            id: tenant.id,
            email: tenant.email,
            name: tenant.name,
            username: tenant.username,
            role: tenant.role,
        }
    }
}