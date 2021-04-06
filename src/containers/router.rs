use crate::containers::handler::static_rocket_route_info_for_create_container;
use crate::containers::handler::static_rocket_route_info_for_get_containers;
use crate::containers::handler::static_rocket_route_info_for_get_real_containers;
use crate::containers::handler::static_rocket_route_info_for_create_real_containers;


pub fn create_containers_routes() -> Vec<rocket::Route> {
    return routes![
            create_container,
            get_containers,
            get_real_containers,
            create_real_containers
        ];
}