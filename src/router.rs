extern crate serde;
extern crate serde_json;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::response::content;
use rocket::response::content::Json;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;

//const
pub const FRONTEND_PATH: &'static str = "frontend/dist/public";


pub struct HitCount(pub AtomicUsize);

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

#[get("/show_count")]
pub fn show_count(hit_count: State<HitCount>) -> content::Html<String> {
    hit_count.0.fetch_add(1, Ordering::Relaxed);
    let msg = "Your visit has been recorded!";
    let count = format!("Visits: {}", count(hit_count));
    content::Html(format!("{}<br /><br />{}", msg, count))
}

#[get("/count")]
pub fn count(hit_count: State<HitCount>) -> String {
    hit_count.0.load(Ordering::Relaxed).to_string()
}


#[get("/ip")]
pub fn get_ip(remote_addr: SocketAddr) -> Json<String>{
    println!("IP: {:?}", remote_addr.ip());
    return Json(format!("IP: {:?}", remote_addr.ip()));
}

#[get("/public/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    // static file manager
    NamedFile::open(Path::new(FRONTEND_PATH).join(file)).ok()
}

pub fn get_routes() -> Vec<rocket::Route> {
    return routes![
        files,
        index,
        json,
        show_count,
        count,
        get_ip
    ];
}

