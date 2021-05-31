#[macro_use] extern crate diesel;
extern crate serde;
#[macro_use] extern crate serde_derive;



use dotenv::dotenv;
use std::env;

use chrono::{Local, NaiveDateTime};

use diesel::deserialize::FromSql;
use diesel::sql_types::Varchar;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::{serialize, deserialize};
use diesel::backend::Backend;
use diesel::prelude::*;

use std::io::Write;

use bcrypt::{hash, DEFAULT_COST};
use serde_json::{Value, json};

table! {
    tenant_configuration (id) {
        id -> Int4,
        tenant_id -> Int4,
        config -> Varchar,
    }
}

table! {
    tenants (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        username -> Varchar,
        password -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        login_session -> Varchar,
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tenants"]
pub struct NewTenant {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tenant_configuration"]
pub struct NewTenantConfiguration {
    pub id: i32,
    pub tenant_id: i32,
    pub config: DBJsonType
}

fn get_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", db_url));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let email = &args[1];
    let username = &args[2];
    let password = &args[3];

    let conn = get_connection();

    // Create Admin tenant change if you want to
    match diesel::insert_into(tenants::table)
        .values(NewTenant {
            id: 0,
            email: email.to_string(),
            name: username.to_string(),
            username: username.to_string(),
            password: hash(password, DEFAULT_COST).unwrap(),
            role: "Admin".to_string(),
            created_at: Local::now().naive_local(),
        }).execute(&conn) {
        Ok(created) => println!("created email: {}, username: {}, password: {}", email, username, password),
        Err(e) => {
            println!("already exist");
            std::process::exit(0);
        }
    };

    // Create the Tenant config for the tenant
    match diesel::insert_into(tenant_configuration::table)
        .values(NewTenantConfiguration {
            id: 0,
            tenant_id: 0,
            config: DBJsonType(json!({}))
        })
        .execute(&conn) {
        Ok(created) => println!("create config"),
        Err(e) => {
            println!("already exist");
            std::process::exit(0);
        }
    }
    std::process::exit(0)
}


// TODO json validation
impl FromSql<Varchar, Pg> for DBJsonType {
    fn from_sql(
        bytes: Option<&<Pg as Backend>::RawValue>,
    ) -> deserialize::Result<Self> {
        let t = <String as FromSql<Varchar, Pg>>::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&t)?))
    }
}

impl ToSql<Varchar, Pg> for DBJsonType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        <String as ToSql<Varchar, Pg>>::to_sql(&s, out)
    }
}

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow)]
#[sql_type = "Varchar"]
pub struct DBJsonType(Value);