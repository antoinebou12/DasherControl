use rocket::local::Client;
use rocket::http::Status;
use crate::create_rocket;

#[test]
fn containers_routes() {
    let client = Client::new(create_rocket()).unwrap();

    let response = client.get("/containers/api/list").dispatch();
    assert_eq!(response.status(), Status::Ok);
}