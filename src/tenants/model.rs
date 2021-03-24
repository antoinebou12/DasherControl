use crate::tenants::schema::*;
use bcrypt::{hash, DEFAULT_COST};
use diesel::PgConnection;
use chrono::Local;
use crate::tenants::error::MyStoreError;
use chrono::NaiveDateTime; 

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "tenants"]
pub struct Tenant {
    #[serde(skip)]
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tenants"]
pub struct NewTenant {
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct RegisterTenant {
    pub email: String,
    pub name: String,
    pub username: String,
    pub role: String,
    pub password: String,
    pub password_confirmation: String
}

impl RegisterTenant {
    pub fn validates(self) ->
     Result<RegisterTenant, MyStoreError> {
         if self.password == self.password_confirmation {
             Ok(self)
         } else {
             Err(
                 MyStoreError::PasswordNotMatch(
                     "Password and Password Confirmation does not match".to_string()
                 )
             )
         }
    }
}


impl Tenant {
    pub fn create(register_tenant: RegisterTenant, connection: &PgConnection) ->
     Result<Tenant, MyStoreError> {
        use diesel::RunQueryDsl;

        return Ok(diesel::insert_into(tenants::table)
            .values(NewTenant {
                email: register_tenant.email,
                name: register_tenant.name,
                username: register_tenant.username,
                role: register_tenant.role,
                password: Self::hash_password(register_tenant.password)?,
                created_at: Local::now().naive_local()
            })
            .get_result(connection)?);
    }

    // This might look kind of weird, 
    // but if something fails it would chain 
    // to our MyStoreError Error, 
    // otherwise it will gives us the hash, 
    // we still need to return a result 
    // so we wrap it in an Ok variant from the Result type. 
    pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
        return Ok(hash(plain, DEFAULT_COST)?);
    }
}