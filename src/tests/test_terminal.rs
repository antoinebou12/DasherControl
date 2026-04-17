use rocket::http::Status;
use rocket::local::blocking::Client;

use crate::create_rocket;

#[test]
fn terminal_ws_rejects_plain_http_without_upgrade() {
    let client = Client::tracked(create_rocket()).expect("tracked client");
    let response = client.get("/terminal/ws?token=invalid").dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}
