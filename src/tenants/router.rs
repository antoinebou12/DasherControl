use crate::tenants::handler;
use crate::tenants::oauth;

pub fn create_tenants_routes() -> Vec<rocket::Route> {
    routes![
        handler::profile,
        handler::get_token,
        handler::all_tenants,
        handler::create_tenant,
        handler::login,
        handler::logout,
        oauth::google_login,
        oauth::google_callback,
        oauth::github_login,
        oauth::github_callback,
    ]
}
