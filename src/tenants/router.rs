use crate::tenants::handler::static_rocket_route_info_for_all_tenants;

pub fn create_tenants_routes() -> Vec<rocket::Route> {
    return routes![
            all_tenants
        ];
}