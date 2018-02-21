use std::string::String;

use cconfig::{Config, ConfigError, Environment, File};

#[derive(Deserialize)]
pub struct GeneralConfig {
    theme: String,
    themes_dir: String,
}

#[derive(Deserialize)]
pub enum DatabaseType {
    PostgreSQL,
    SQLite3,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    database_type: DatabaseType,
}

#[derive(Deserialize)]
pub struct StoryArchiveConfig {
    general: GeneralConfig,
    database: DatabaseConfig,
}

impl StoryArchiveConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(Environment::with_prefix("STORYARCHIVE"));

        s.merge(File::with_name("StoryArchive"));

        s.try_into()
    }
}
