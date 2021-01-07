#![feature(proc_macro_hygiene, decl_macro, bindings_after_at, backtrace)]

pub use common;
use rocket::{catchers, get, routes};
use rocket_contrib::serve::StaticFiles;

pub mod auth;
mod catchers;
pub mod db;
mod endpoints;

pub fn build_rocket() -> rocket::Rocket {
    let client = db::DBClient::init("mongodb://localhost:27017/");
    let routes = routes![
        endpoints::signup::signup_endpoint,
        endpoints::login::login_endpoint,
        endpoints::user::self_endpoint,
    ];
    rocket::ignite()
        .manage(client.get_database("appdb"))
        .mount("/", routes)
        .mount("/public", StaticFiles::from("/static"))
        .register(catchers![
            catchers::not_found,
            catchers::internal_server_error,
            catchers::unauthorized
        ])
}
