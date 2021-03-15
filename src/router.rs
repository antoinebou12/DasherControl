extern crate serde;
extern crate serde_json;

use rocket_contrib::templates::Template;

pub const FRONTEND_PATH: &'static str = "frontend/dist";

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

#[derive(Serialize, Debug)]
struct JsonMessage {
    status: u16,
    msg: &'static str
}

#[get("/")]
pub fn index() -> Template {
    let context = TemplateContext { name: "index".to_string(), items: vec![] };
    Template::render("index", &context)
}

#[get("/json")]
pub fn json() -> String {
    let message = JsonMessage { 
        status: 200,
        msg: "json exemple" 
    };
    serde_json::to_string(&message).unwrap()
}