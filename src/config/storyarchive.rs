use std::string::String;

use cconfig::{Config, ConfigError, Environment, File};

/// StoryArchive's master config struct.
///
/// # Fields
///   * ```general```: Contains basic flags like themes.
///     (see ```GeneralConfig``` struct)
///   * ```database```: Contains database conection information.
///     (see ```DatabaseConfig``` struct)
#[derive(Clone, Serialize, Deserialize)]
pub struct StoryArchiveConfig {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
}

/// The general category in StoryArchive's config.
///
/// # Fields
///   * ```theme```: The current theme being used.
///   * ```themes_dir```: The path to where themes are being stored.
#[derive(Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub theme: String,
    pub themes_dir: String,
}

/// The database category in StoryArchive's config.
///
/// # Fields
///   * ```database_name```: The name of the database that MongoDB will use.
///   * ```database_url```: The MongoDB connection URL.
#[derive(Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_name: String,
    pub database_url: String,
}

impl StoryArchiveConfig {
    /// Call the Config crate to create StoryArchive's config from files and the env.
    ///
    /// If there is an error ```ConfigError``` is thrown.
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Combine environmental variabled into the empty config.
        //
        // Thows ```ConfigError``` on an error.
        s.merge(Environment::with_prefix("STORYARCHIVE"))?;

        // Combine the normally empty config with the config file.
        //
        // Thows ```ConfigError``` on an error.
        s.merge(File::with_name("StoryArchive"))?;

        // Attempt to turn the gathered config into the config struct.
        //   (see ```StoryArchiveConfig``` struct)
        s.try_into()
    }
}
