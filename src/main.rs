#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate config as cconfig;
extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               index
        ])
        .launch();
}
