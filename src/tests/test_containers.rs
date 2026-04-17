use rocket::http::Status;
use rocket::local::blocking::Client;
use serde_json::Value;

use crate::create_rocket;

#[test]
fn containers_routes() {
    let client = Client::tracked(create_rocket()).expect("rocket");

    let response = client.get("/containers/api/list").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let list: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert!(list.is_array());

    let response = client.get("/containers/api/docker_snapshot").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let snap: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert!(snap.is_array());
}