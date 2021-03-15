use rocket_contrib::databases::{database, diesel};

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);