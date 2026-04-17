#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

use std::sync::atomic::AtomicUsize;

use rocket::Request;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use rocket_dyn_templates::Template;
use serde::Serialize;

pub mod tenants;
pub mod workspaces;
pub mod router;
pub mod fairing;
pub mod db;
pub mod schema;
pub mod containers;
pub mod caddy;
pub mod terminal;
pub mod tests;
pub mod system;
pub mod assistant;

#[derive(Serialize)]
struct NotFoundCtx<'a> {
    path: &'a str,
}

#[catch(404)]
async fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "errors/404",
        &NotFoundCtx {
            path: req.uri().path().as_str(),
        },
    )
}

#[catch(409)]
fn conflict() -> Json<Value> {
    Json(json!({ "error": "conflict" }))
}

pub fn create_rocket() -> rocket::Rocket<rocket::Build> {
    let routes = router::get_routes();
    let tenants_routes = tenants::router::create_tenants_routes();
    let workspaces_routes = workspaces::router::create_workspaces_routes();
    let containers_routes = containers::router::create_containers_routes();
    let caddy_routes = caddy::router::create_caddy_routes();
    let terminal_routes = terminal::router::routes();
    let system_routes = system::router::create_system_routes();
    let assistant_routes = assistant::router::create_assistant_routes();

    rocket::build()
        .manage(db::init_pool())
        .manage(system::SystemMetricsState::default())
        .attach(fairing::RequestLogging)
        .attach(fairing::CORS())
        .mount("/", routes)
        .mount("/tenants", tenants_routes)
        .mount("/workspaces", workspaces_routes)
        .mount("/containers", containers_routes)
        .mount("/caddy", caddy_routes)
        .mount("/terminal", terminal_routes)
        .mount("/system", system_routes)
        .mount("/assistant", assistant_routes)
        .manage(router::HitCount(AtomicUsize::new(0)))
        .attach(Template::fairing())
        .register("/", catchers![not_found, conflict])
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("dasher_control=info,rocket=warn"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    init_tracing();
    create_rocket().launch().await?;
    Ok(())
}
