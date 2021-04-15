use crate::containers::handler::static_rocket_route_info_for_create_container;
use crate::containers::handler::static_rocket_route_info_for_get_containers;
use crate::containers::handler::static_rocket_route_info_for_get_docker_containers;
use crate::containers::handler::static_rocket_route_info_for_create_docker_containers;
use crate::containers::handler::static_rocket_route_info_for_restart_docker_container;
use crate::containers::handler::static_rocket_route_info_for_stop_docker_container;


pub fn create_containers_routes() -> Vec<rocket::Route> {
    return routes![
            create_container,
            get_containers,
            get_docker_containers,
            create_docker_containers,
            restart_docker_container,
            stop_docker_container
        ];
}