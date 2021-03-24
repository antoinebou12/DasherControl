#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::tenants::model::Tenant;
use crate::tenants::schema::tenants;



pub fn all(conn: &PgConnection) -> QueryResult<Vec<Tenant>> {
    return tenants::table.load::<Tenant>(&*conn);
}