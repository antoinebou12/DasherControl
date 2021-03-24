#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;


use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

extern crate serde;
extern crate serde_json;

use rocket::Request;
use rocket_contrib::templates::Template;

mod tenants;
mod router;
mod fairing;
mod db;

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    // 404
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("errors/404", &map)
}


fn create_rocket() -> rocket::Rocket {
    // routes
    let routes = router::get_routes();
    //route tenants
    let tenants_routes = tenants::router::create_tenants_routes();

    // CORS
    // let cors = create_cors_rocket();

    // Start Rocket app
    return rocket::ignite()
     // .attach(db::DbConnRocket::fairing())
        .manage(db::init_pool())
        .attach(fairing::CORS())
        .mount("/", routes)
        .mount("/tenants", tenants_routes)
        .manage(router::HitCount(AtomicUsize::new(0)))
        .attach(Template::fairing())
        .register(catchers![not_found]);
}

fn main() {
    // Start rocket with CORS
    create_rocket().launch();
}
