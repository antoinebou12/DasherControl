use serde::Serialize;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::State;
pub const FRONTEND_PATH: &str = "frontend/dist/public";
pub const FRONTEND_INDEX: &str = "frontend/dist/public/index.html";

pub struct HitCount(pub AtomicUsize);

#[derive(Serialize, Debug)]
struct JsonMessage {
    status: u16,
    msg: &'static str,
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(FRONTEND_INDEX)).await.ok()
}

#[get("/json")]
pub async fn json() -> String {
    let message = JsonMessage {
        status: 200,
        msg: "json exemple",
    };
    serde_json::to_string(&message).unwrap()
}

#[get("/show_count")]
pub async fn show_count(hit_count: &State<HitCount>) -> RawHtml<String> {
    hit_count.0.fetch_add(1, Ordering::Relaxed);
    let msg = "Your visit has been recorded!";
    let n = hit_count.0.load(Ordering::Relaxed);
    RawHtml(format!("{}<br /><br />Visits: {}", msg, n))
}

#[get("/count")]
pub async fn count_view(hit_count: &State<HitCount>) -> String {
    hit_count.0.load(Ordering::Relaxed).to_string()
}

#[get("/ip")]
pub async fn get_ip(remote_addr: SocketAddr) -> Json<String> {
    Json(format!("IP: {:?}", remote_addr.ip()))
}

#[get("/public/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(FRONTEND_PATH).join(file)).await.ok()
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        files,
        index,
        json,
        show_count,
        count_view,
        get_ip
    ]
}
