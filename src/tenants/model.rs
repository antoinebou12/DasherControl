use bcrypt::{DEFAULT_COST, hash, verify};
use uuid::Uuid;
use chrono::{Local, NaiveDateTime};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, select};
use diesel::expression::dsl::exists;

use crate::schema::tenants;
use crate::schema::login_history;
use crate::schema::tenants::dsl::{username, email, login_session, id};
use crate::tenants::error::MyError;
use crate::tenants::token::Claims;

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
    #[serde(skip)]
    pub login_session: String,
}

impl Tenant {
    pub fn create(register_tenant: RegisterTenant, conn: &PgConnection) ->
    Result<bool, MyError> {
        return Ok(diesel::insert_into(tenants::table)
            .values(NewTenant {
                email: register_tenant.email,
                name: register_tenant.name,
                username: register_tenant.username,
                password: Self::hash_password(register_tenant.password)?,
                role: register_tenant.role,
                created_at: Local::now().naive_local(),
            })
            .execute(conn)
            .is_ok());
    }

    pub fn hash_password(plain: String) -> Result<String, MyError> {
        return Ok(hash(plain, DEFAULT_COST)?);
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Tenant>> {
        return tenants::table.load::<Tenant>(&*conn);
    }

    pub fn is_valid_login_session(tenant_token: &Claims, conn: &PgConnection) -> bool {
        tenants::table
            .filter(id.eq(&tenant_token.sub))
            .filter(login_session.eq(&tenant_token.login_session))
            .get_result::<Tenant>(conn)
            .is_ok()
    }

    pub fn find_tenant_by_username(name: &str, conn: &PgConnection) -> Option<Tenant> {
        let result_tenant = tenants::table.filter(username.eq(name)).get_result::<Tenant>(conn);
        if let Ok(tenant) = result_tenant {
            Some(tenant)
        } else {
            None
        }
    }

    pub fn find_tenant_by_email(mail: &str, conn: &PgConnection) -> Option<Tenant> {
        let result_tenant = tenants::table.filter(email.eq(mail)).get_result::<Tenant>(conn);
        if let Ok(tenant) = result_tenant {
            Some(tenant)
        } else {
            None
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn update_login_session_to_db(name: &str, session: &str, conn: &PgConnection) -> bool {
        if let Some(tenant) = Tenant::find_tenant_by_username(name, conn) {
            diesel::update(tenants::table.find(tenant.id))
                .set(login_session.eq(session.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
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
        let mut tenant = match tenants::table
            .filter(email.eq(&self.email))
            .or_filter(username.eq(&self.username))
            .first::<Tenant>(&*conn) {
            Ok(tenant) => tenant,
            Err(e) => return Err(MyError::NotFoundTenant(e.to_string()))
        };

        let valid = verify(&self.password, &tenant.password).unwrap();

        if valid {
            if let Some(login_history) = LoginHistory::create(&tenant.username, conn) {
                if !LoginHistory::save_login_history(login_history, conn) {
                    return Err(MyError::LoginHistoryError("Error saving login history".to_string()));
                }
                let session = Tenant::generate_login_session();
                Tenant::update_login_session_to_db(&tenant.username, &session, conn);
                tenant.login_session = session;
                return Ok(tenant);
            }
        } else {
            return Err(MyError::LoginHistoryError("Error with login history".to_string()));
        }
        return Err(MyError::WrongPassword(
            "Wrong Password".to_string(),
        ));
    }
}


#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(Tenant)]
#[table_name = "login_history"]
pub struct LoginHistory {
    pub id: i32,
    pub tenant_id: i32,
    pub login_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct LoginHistoryInsertable {
    pub tenant_id: i32,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub fn create(name: &str, conn: &PgConnection) -> Option<LoginHistoryInsertable> {
        if let Some(tenant) = Tenant::find_tenant_by_username(name, conn) {
            Some(LoginHistoryInsertable {
                tenant_id: tenant.id,
                login_timestamp: Local::now().naive_local(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(insert_record: LoginHistoryInsertable, conn: &PgConnection) -> bool {
        diesel::insert_into(login_history::table)
            .values(&insert_record)
            .execute(conn)
            .is_ok()
    }
}

