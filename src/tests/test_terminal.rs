use rocket::local::Client;
use rocket::http::Status;
use crate::create_rocket;

#[test]
fn terminals_routes() {
    let client = Client::new(create_rocket()).unwrap();
}