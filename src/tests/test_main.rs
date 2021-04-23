use rocket::local::Client;
use rocket::http::Status;
use crate::create_rocket;

#[test]
fn main_routes() {
    let client = Client::new(create_rocket()).unwrap();

    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.get("/bad_url").dispatch();
    assert_eq!(response.status(), Status::NotFound);

    // let mut response = client.get("/ip").dispatch();
    // assert_eq!(response.status(), Status::Ok);

    let response = client.get("/public/robots.txt").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.get("/public/error.error").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
