use crate::tenants::handler::*;

pub fn create_tenants_routes() -> Vec<rocket::Route> {
    return routes![
            all_tenants,
            create_tenant
        ];
}