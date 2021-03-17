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
pub struct InsertableTenant {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: String,
}

impl InsertableTenant {
    pub fn from_tenant(tenant: Tenant) -> InsertableTenant {
        InsertableTenant {
            id: tenant.id,
            email: tenant.email,
            name: tenant.name,
            username: tenant.username,
            password: tenant.password,
            role: tenant.role,
        }
    }
}
