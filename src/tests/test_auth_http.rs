use rocket::http::{ContentType, Header, Status};
use rocket::local::blocking::Client;

use crate::create_rocket;

#[test]
fn caddy_generate_requires_bearer() {
    let client = Client::tracked(create_rocket()).expect("client");
    let body = r#"{"sites":[{"hostname":"a.example","upstream":"127.0.0.1:1"}]}"#;
    let response = client
        .post("/caddy/api/generate")
        .header(ContentType::JSON)
        .body(body)
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn docker_snapshot_post_requires_bearer() {
    let client = Client::tracked(create_rocket()).expect("client");
    let response = client.post("/containers/api/docker_snapshot").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn workspaces_list_requires_bearer() {
    let client = Client::tracked(create_rocket()).expect("client");
    let response = client.get("/workspaces/api/list").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn workspaces_list_rejects_malformed_bearer() {
    let client = Client::tracked(create_rocket()).expect("client");
    let response = client
        .get("/workspaces/api/list")
        .header(Header::new("Authorization", "Bearer not-a-jwt"))
        .dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}
