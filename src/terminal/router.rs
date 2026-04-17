use crate::terminal::handler;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![handler::terminal_ws]
}
