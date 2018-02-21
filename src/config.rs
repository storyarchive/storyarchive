use cconfig::{Config, ConfigError, Environment, File};

#[derive(Deserialize)]
pub struct StoryArchiveConfig {

}

impl StoryArchiveConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::new();

        s.try_into()
    }
}
