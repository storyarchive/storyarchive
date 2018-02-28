#![feature(plugin)]
#![plugin(rocket_codegen)]


#[macro_use(bson, doc)]
extern crate bson;
extern crate config as cconfig;
extern crate fluent;
extern crate mongodb;
extern crate rocket;
extern crate rocket_contrib;
extern crate sass_rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;


mod config;
mod extras;
mod helpers;
mod modules;
mod models;


use std::path::{Path, PathBuf};
use std::string::String;
use std::sync::Arc;

use config::storyarchive::StoryArchiveConfig;
use config::theme::ThemeConfig;
use extras::sass::Sass;
use helpers::db::init_db;
use modules::hub;

use rocket::State;
use rocket::response::NamedFile;
use rocket_contrib::Template;


const VERSION: &'static str = env!("CARGO_PKG_VERSION");


#[get("/<file..>", rank = 2)]
fn assets(
    file: PathBuf,
) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets").join(file)).ok()
}

#[get("/theme/<file..>", rank = 1)]
fn assets_theme(
    file: PathBuf,
    storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
) -> Option<NamedFile> {
    NamedFile::open(Path::new(
        &format!(
            "{}/{}/{}",
            &storyarchive_config.general.themes_dir.clone(),
            &storyarchive_config.general.theme.clone(),
            &theme_config.theme.assets.clone()
        )
    ).join(file)).ok()
}

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


    let mut sass = Sass::new(
        config.clone(),
        theme_config.clone()
    ).map_err(|e| e.to_string())?;

    sass = sass.write()
        .map_err(|e| e.to_string())?;

    let rocket = rocket::ignite()
        .mount("/", hub::routes())
        .mount("assets/", routes![assets, assets_theme])
        .attach(Template::fairing())
        .manage(Arc::clone(&client))
        .manage(Arc::clone(&db))
        .manage(config.clone())
        .manage(theme_config.clone())
        .manage(sass.clone());

    Ok(rocket)
}

fn main() {
    let rocket = read().unwrap();
    rocket.launch();
}
