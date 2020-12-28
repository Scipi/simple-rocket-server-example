#![feature(proc_macro_hygiene, decl_macro)]

pub use common;
use rocket::{catchers, get, routes};
use rocket_contrib::serve::StaticFiles;

pub mod auth;
mod catchers;
pub mod db;
mod endpoints;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[get("/hello/<first_name>?<last_name>&<greeting_noun>")]
fn hello_full(first_name: String, last_name: String, greeting_noun: Option<String>) -> String {
    let greeting_noun = greeting_noun
        .map(|noun| format!("*{}* ", noun))
        .unwrap_or_else(|| "".into());
    format!("{}Hello, {} {}!", greeting_noun, first_name, last_name)
}

pub fn build_rocket() -> rocket::Rocket {
    let client = db::DBClient::init("mongodb://localhost:27017/");
    let routes = routes![
        index,
        hello,
        hello_full,
        endpoints::signup::signup_endpoint,
        endpoints::login::login_endpoint
    ];
    rocket::ignite()
        .manage(client.get_app_database("appdb"))
        .mount("/", routes)
        .mount("/public", StaticFiles::from("/static"))
        .register(catchers![
            catchers::not_found,
            catchers::internal_server_error
        ])
}
