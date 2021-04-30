use std::env;
use std::ops::Deref;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use rocket::{Outcome, Request};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::State;
use rocket_contrib::databases::diesel;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    return Pool::new(manager).expect("db pool");
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

