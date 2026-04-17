use crate::system::handler;

pub fn create_system_routes() -> Vec<rocket::Route> {
    routes![handler::get_metrics, handler::network_speed_test]
}
