#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate config as cconfig;
extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;

mod config;
mod helpers;
mod hub;
mod theme;

fn main() {
    let config = config::StoryArchiveConfig::new();

    rocket::ignite()
        .mount("/", hub::routes())
        .launch();
}
