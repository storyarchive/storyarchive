use std::string::String;

use cconfig::{Config, ConfigError, Environment, File};

#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    pub theme: String,
    pub themes_dir: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_name: String,
    pub database_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct StoryArchiveConfig {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
}

impl StoryArchiveConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(Environment::with_prefix("STORYARCHIVE"))?;

        s.merge(File::with_name("StoryArchive"))?;

        s.try_into()
    }
}
