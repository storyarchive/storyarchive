#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate config as cconfig;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;

mod config;
mod helpers;
mod hub;
mod theme;

fn main() {
    let config = config::StoryArchiveConfig::new().unwrap();
    let theme_config = theme::ThemeConfig::new(&config).unwrap();

    rocket::ignite()
        .mount("/", hub::routes())
        .launch();
}
