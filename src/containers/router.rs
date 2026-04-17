use crate::containers::handler;

pub fn create_containers_routes() -> Vec<rocket::Route> {
    routes![
        handler::create_container_route,
        handler::get_containers,
        handler::get_docker_containers,
        handler::get_docker_snapshot,
        handler::post_docker_snapshot,
        handler::create_docker_containers,
        handler::restart_docker_container,
        handler::stop_docker_container,
        handler::docker_export_zip,
        handler::post_docker_import_zip,
    ]
}
