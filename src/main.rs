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


/// Base StoryArchive assets.
///
/// Due to ordering and design theme assets come before base assets.
///
/// # Parameters
///   * ```file```: The base assest file requested.
#[get("/<file..>", rank = 2)]
fn assets(
    file: PathBuf,
) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets").join(file)).ok()
}

/// Any assets that come from themes.
///
/// Due to ordering and design theme assets come before base assets.
///
/// # Parameters
///   * ```file```: The Theme assest file requested.
///   * ```storyarchive_config```: The cloned instance of the StoryArchive config.
///   * ```theme_config```: The cloned instance of the Theme config.
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

/// A wrapper function that will pass any error to its output.
///
/// # Example
/// ```rust
/// let rocket = match read() {
///     Err(e) => panic!("{}", e),
///     Ok(r) => r,
/// };
/// 
/// rocket.launch();
/// ```
fn read() -> Result<rocket::Rocket, String> {
    // Load in the config from ```StoryArchive.toml```.
    //
    // Throw error as string to Result.
    let config = StoryArchiveConfig::new()
        .map_err(|e| e.to_string())?;

    // Load in the config from ```Theme.toml``` of the theme StoryArchive is configured to use.
    //
    // Throw error as string to Result.
    let theme_config = ThemeConfig::new(
        config.general.theme.clone(),
        config.general.themes_dir.clone(),
    ).map_err(|e| e.to_string())?;

    // Attempt to connect to database, gaining access to the client itself and the database.
    //
    // Throw error as string to Result.
    let (client, db) = init_db(
        &config.database.database_url.clone(),
        &config.database.database_name.clone(),
    ).map_err(|e| e.to_string())?;

    // Check theme for scss and compile it.
    //
    // Throw error as string to Result.
    let mut sass = Sass::new(
        config.clone(),
        theme_config.clone()
    ).map_err(|e| e.to_string())?;

    // Write compiled scss to theme assets folder.
    //
    // Throw error as string to Result.
    sass = sass.write()
        .map_err(|e| e.to_string())?;

    // Start and mount Rocket.
    //
    // Any errors at this point are handled by the caller.
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
    let rocket = match read() {
        Err(e) => panic!("{}", e),
        Ok(r) => r,
    };

    rocket.launch();
}
