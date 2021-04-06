use std::io::Cursor;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
            response.set_header(Header::new("X-Frame-Options", "Allow"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}


// fn create_cors_rocket() -> rocket_cors::Cors {
//     allowed_origins = AllowedOrigins::all();

//     let options = rocket_cors::Cors {
//         allowed_origins: allowed_origins,
//         allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
//         allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
//         allow_credentials: true,
//         ..Default::default()
//     };
//     return options
// } 