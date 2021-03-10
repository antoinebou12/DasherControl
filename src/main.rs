#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

//#use rocket::http::uri::Path;
//#use std::path::Path;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

//#[get("/<path..>")]
//fn files(path: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("static/").join(path)).ok()
//}

//#[get("/admin")]
//fn admin_panel(admin: AdminUser) -> &'static str {

//}

//#[get("/admin", rank = 2)]
//fn admin_panel_user(user: User) -> &'static str {

//}

//#[get("/admin", rank = 3)]
//fn admin_panel_redirect() -> &'static str {
   
//}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
