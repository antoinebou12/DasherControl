use crate::caddy::handler;

pub fn create_caddy_routes() -> Vec<rocket::Route> {
    routes![handler::generate_caddy]
}
