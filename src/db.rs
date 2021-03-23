use rocket_contrib::databases::diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;



const PgConnection conn= create_connection()

fn database_url() -> String {
    dotenv().ok();
    return env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

fn create_connection() -> PgConnection {
    PgConnection::establish(&database_url()).expect("Error connecting to database!")
}



// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
//         let pool = request.guard::<State<Pool>>()?;
//         match pool.get() {
//             Ok(conn) => Outcome::Success(DbConn(conn)),
//             Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//         }
//     }
// }

// impl Deref for DbConn {
//     type Target = PgConnection;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
