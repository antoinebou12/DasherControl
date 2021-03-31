use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Local, NaiveDateTime};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, select};
use diesel::expression::dsl::exists;

use crate::tenants::error::MyError;
use crate::tenants::schema::tenants;
use crate::tenants::schema::tenants::dsl::email;
use crate::tenants::schema::tenants::dsl::username;

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
    pub fn create(register_tenant: RegisterTenant, conn: &PgConnection) ->
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
            .get_result(conn)?);
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
    pub fn validates(self, conn: &PgConnection) ->
    Result<RegisterTenant, MyError> {
        let tenant = select(
            exists(
                tenants::table.filter(email.eq(&self.email)
                )
            )
        ).get_result(*&conn);

        if tenant.eq(&Ok(true)) {
            return Err(MyError::WrongPassword(
                "Wrong Password".to_string(),
            ));
        };

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
                "Wrong Password".to_string(),
            ))
        }
    }
}


#[derive(Deserialize)]
pub struct AuthTenant {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl AuthTenant {
    pub fn login(&self, conn: &PgConnection) ->
    Result<Tenant, MyError> {
        let tenant = tenants::table
            .filter(email.eq(&self.email))
            .or_filter(username.eq(&self.username))
            .first::<Tenant>(&*conn).expect("error");

        let valid = verify(&self.password, &tenant.password).unwrap();

        if valid {
            return Ok(tenant);
        }
        return Err(MyError::WrongPassword(
            "Wrong Password".to_string(),
        ));
    }
}