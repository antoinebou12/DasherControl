use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Local, NaiveDateTime};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::tenants::error::MyError;
use crate::tenants::schema::*;
use crate::tenants::schema::tenants::dsl::email;

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
    pub created_at: NaiveDateTime,
}

impl Tenant {
    pub fn create(register_tenant: RegisterTenant, connection: &PgConnection) ->
    Result<Tenant, MyError> {
        return Ok(diesel::insert_into(tenants::table)
            .values(NewTenant {
                email: register_tenant.email,
                name: register_tenant.name,
                username: register_tenant.username,
                role: register_tenant.role,
                password: Self::hash_password(register_tenant.password)?,
                created_at: Local::now().naive_local(),
            })
            .get_result(connection)?);
    }

    pub fn hash_password(plain: String) -> Result<String, MyError> {
        return Ok(hash(plain, DEFAULT_COST)?);
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tenants"]
pub struct NewTenant {
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RegisterTenant {
    pub email: String,
    pub name: String,
    pub username: String,
    pub role: String,
    pub password: String,
    pub password_confirmation: String,
}

impl RegisterTenant {
    pub fn validates(self) ->
    Result<RegisterTenant, MyError> {
        let password_are_equal = self.password == self.password_confirmation;
        let password_not_empty = self.password.len() > 0;
        if password_are_equal && password_not_empty {
            Ok(self)
        } else if !password_are_equal {
            Err(MyError::PasswordNotMatch(
                "Password and Password Confirmation does not match".to_string(),
            ))
        } else {
            Err(MyError::WrongPassword(
                "Wrong Password, check it is not empty".to_string(),
            ))
        }
    }
}


#[derive(Deserialize)]
pub struct AuthTenant {
    pub email: String,
    pub password: String,
}

impl AuthTenant {
    pub fn login(&self, conn: &PgConnection) -> Tenant {
        let tenant = tenants::table
            .filter(email.eq(&self.email))
            .first::<Tenant>(conn).expect("error");

        let valid = match verify(&self.password, &tenant.password) {
            Ok(valid) => valid,
            Err(_) => panic!()
        };

        if valid {
            return tenant;
        }
        return tenant;
    }
}