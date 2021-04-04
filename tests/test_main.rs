// importing common module.
mod common;

use rocket::local::Client;
use rocket::http::Status;

#[test]
fn test_index() {
	let client = Client::new(rocketrustvuejs::create_rocket().launch()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}


#[test]
fn test_404() {
}