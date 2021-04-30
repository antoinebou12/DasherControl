#![feature(proc_macro_hygiene, decl_macro, plugin)]
#![feature(in_band_lifetimes)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;


use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

use rocket::Request;
use rocket_contrib::templates::Template;
use crate::terminal::websocket;


pub mod tenants;
pub mod workspaces;
pub mod router;
pub mod fairing;
pub mod db;
pub mod schema;
pub mod containers;
pub mod terminal;
pub mod tests;


#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    // 404
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("errors/404", &map)
}


pub fn create_rocket() -> rocket::Rocket {
    // routes
    let routes = router::get_routes();
    //route tenants
    let tenants_routes = tenants::router::create_tenants_routes();
    // routes workspaces
    let workspaces_routes = workspaces::router::create_workspaces_routes();

    // routes containers
    let containers_routes = containers::router::create_containers_routes();

    // CORS
    // let cors = create_cors_rocket();

    // Start Rocket app
    return rocket::ignite()
        .manage(db::init_pool())
        .attach(fairing::CORS())

        .mount("/", routes)
        .mount("/tenants", tenants_routes)
        .mount("/workspaces", workspaces_routes)
        .mount("/containers", containers_routes)

        .manage(router::HitCount(AtomicUsize::new(0)))
        .attach(Template::fairing())

        .register(catchers![not_found]);
}

pub fn main() {
    // Start rocket
    create_rocket().launch();
}

