// #![feature(plugin)]
// #![plugin(rocket_codegen)]

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket;

extern crate rocket_cors;
extern crate serde;
extern crate serde_json;

use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("views/dist/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("views/dist/").join(file)).ok()
}

#[derive(Serialize, Debug)]
struct Test {
    message: &'static str
}

#[get("/hello")]
fn hello() -> String {
    let hello_message = Test { message: "hello world" };
    serde_json::to_string(&hello_message).unwrap()
}

//#[get("/<path..>")]
//fn files(path: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("static/").join(path)).ok()
//}

//#[get("/admin")]
//fn admin_panel(admin: AdminUser) -> &'static str {

//}

//#[get("/admin", rank = 2)]
//fn admin_panel_user(user: User) -> &'static str {

//}

//#[get("/admin", rank = 3)]
//fn admin_panel_redirect() -> &'static str {
   
//}


fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://0.0.0.0:8000", "http://0.0.0.0:8081"]);
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![index, files, hello])
        .attach(options)
        .launch();
}