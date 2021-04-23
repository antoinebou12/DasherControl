use rocket::local::Client;
use rocket::http::Status;
use crate::create_rocket;

#[test]
fn tenants_routes() {
    let client = Client::new(create_rocket()).unwrap();
}