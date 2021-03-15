// importing common module.
mod common;

use rocket::local::Client;
use rocket::http::Status;

#[test]
fn index() {
	let client = Client::new(main()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_404() {
    // Check that the error catcher works.
    dispatch!(Get, "/Error/", |client, response| {
        let mut map = std::collections::HashMap::new();
        map.insert("path", "/Error/");

        let expected = Template::show(client.rocket(), "errors/404", &map).unwrap();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.into_string(), Some(expected));
    });
}
