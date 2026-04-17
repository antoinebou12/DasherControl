use rocket::http::Status;
use rocket::local::blocking::Client;
use serde_json::Value;

use crate::create_rocket;

#[test]
fn main_routes() {
    let client = Client::tracked(create_rocket()).expect("rocket");

    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.get("/bad_url").dispatch();
    assert_eq!(response.status(), Status::NotFound);

    let response = client.get("/ip").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().expect("body");
    let v: Value = serde_json::from_str(&body).expect("json");
    assert!(v.as_str().unwrap_or("").contains("IP:"));

    let response = client.get("/json").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let v: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(v["status"], 200);

    let response = client.get("/public/robots.txt").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.get("/public/error.error").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
