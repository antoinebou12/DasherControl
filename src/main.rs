// #![feature(plugin)]
// #![plugin(rocket_codegen)]

#![feature(proc_macro_hygiene, decl_macro, plugin)]

// use std::io;
// use std::env;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::Request;
use rocket::http::Method;
use rocket::response::NamedFile;

use rocket_contrib::templates::Template;
// use rocket_contrib::serve::StaticFiles;

extern crate rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod router;

mod db;


#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    // static file manager
    NamedFile::open(Path::new(router::FRONTEND_PATH).join(file)).ok()
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    // 404
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("errors/404", &map)
}

fn create_cors() -> rocket_cors::Cors {
    let urls = &["http://0.0.0.0:8000", "http://0.0.0.0:8081"];
    let (allowed_origins, failed_origins) = AllowedOrigins::some(urls);
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };
    return options
} 

fn start_rocket(options: rocket_cors::Cors) -> () {
    // routes
    let routes = routes![
        router::index,
        router::json,
        files
    ];

    // Start Rocket app
    rocket::ignite()
        .mount("/", routes)
        .attach(options)
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}

fn main() {
    // CORS
    let options = create_cors();

    db::establish_connection();

    // Start rocket with CORS
    start_rocket(options);
}
