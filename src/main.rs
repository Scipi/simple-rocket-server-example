#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;

mod db;

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

fn main() {
    let routes = routes![index, hello, hello_full];
    rocket::ignite()
        .manage(db::DBClient::init("mongodb://localhost:27107".into()))
        .mount("/", routes)
        .mount("/public", StaticFiles::from("/static"))
        .launch();
}
