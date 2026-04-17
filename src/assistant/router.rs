use crate::assistant::handler;

pub fn create_assistant_routes() -> Vec<rocket::Route> {
    routes![handler::chat, handler::status]
}
