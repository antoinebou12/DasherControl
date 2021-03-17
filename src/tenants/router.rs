use crate::tenants;
use rocket;

pub fn create_tenants_routes() -> Vec<rocket::Route> {
    return routes![
            tenants::handler::all,
            tenants::handler::get,
            tenants::handler::post,
            tenants::handler::put,
            tenants::handler::delete
            ]
}