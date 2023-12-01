#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

mod controllers;
mod db;
mod models;
mod view_renderer;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", controllers::get_routes())
        .mount("/css", FileServer::from("css"))
        .mount("/js", FileServer::from("js"))
}
