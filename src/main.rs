#![feature(plugin)]
#![plugin(rocket_codegen)]


#[macro_use(bson, doc)]
extern crate bson;
extern crate config as cconfig;
extern crate mongodb;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;


mod config;
mod helpers;
mod hub;


use std::string::String;
use std::sync::Arc;

use config::storyarchive::StoryArchiveConfig;
use config::theme::ThemeConfig;
use helpers::db::init_db;

use rocket_contrib::Template;


const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn read() -> Result<rocket::Rocket, String> {
    let config = StoryArchiveConfig::new()
        .map_err(|e| e.to_string())?;

    let theme_config = ThemeConfig::new(
        config.general.theme.clone(),
        config.general.themes_dir.clone(),
    ).map_err(|e| e.to_string())?;

    let (client, db) = init_db(
        &config.database.database_url.clone(),
        &config.database.database_name.clone(),
    ).map_err(|e| e.to_string())?;

    let rocket = rocket::ignite()
        .mount("/", hub::routes())
        .attach(Template::fairing())
        .manage(Arc::clone(&client))
        .manage(Arc::clone(&db))
        .manage(config)
        .manage(theme_config);

    Ok(rocket)
}

fn main() {
    let rocket = read().unwrap();
    rocket.launch();
}
