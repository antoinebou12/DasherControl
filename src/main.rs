#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
// #[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


// use std::io;
// use std::env;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

extern crate serde;
extern crate serde_json;

use rocket::Request;
use rocket::http::Method;
use rocket::response::NamedFile;

use rocket_contrib::templates::Template;

extern crate rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod tenants;
mod router;
mod db;




#[get("/public/<file..>")]
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

fn get_routes() -> Vec<rocket::Route> {
    return routes![
        router::index,
        router::json,
        router::show_count,
        router::count,
        files
    ];
}

fn start_rocket() -> rocket::Rocket {
    // routes
    let routes = get_routes();
    //route tenants
    let tenants_routes = tenants::router::create_tenants_routes();

    // CORS
    let cors = create_cors();

    // Start Rocket app
    return rocket::ignite()
     // .attach(db::DbConnRocket::fairing())
        .manage(db::init_pool())
        .attach(cors)
        .mount("/", routes)
        .mount("/tenant", tenants_routes)
        .manage(router::HitCount(AtomicUsize::new(0)))
        .attach(Template::fairing())
        .register(catchers![not_found]);
}

fn main() {
    // Start rocket with CORS
    start_rocket().launch();
}
