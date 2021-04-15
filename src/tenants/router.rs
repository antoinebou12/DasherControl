use crate::tenants::handler::static_rocket_route_info_for_all_tenants;
use crate::tenants::handler::static_rocket_route_info_for_create_tenant;
use crate::tenants::handler::static_rocket_route_info_for_login;
use crate::tenants::handler::static_rocket_route_info_for_logout;
use crate::tenants::handler::static_rocket_route_info_for_get_token;

pub fn create_tenants_routes() -> Vec<rocket::Route> {
    return routes![
            all_tenants,
            create_tenant,
            login,
            logout,
            get_token
        ];
}