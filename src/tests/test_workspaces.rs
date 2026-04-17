use rocket::local::blocking::Client;

use crate::create_rocket;

#[test]
fn workspaces_routes() {
    let _client = Client::tracked(create_rocket()).expect("rocket");
}