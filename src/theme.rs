use cconfig::{Config, ConfigError, File};

use config;

#[derive(Deserialize)]
pub struct ThemeConfig {

}

impl ThemeConfig {
    pub fn new(config: &config::StoryArchiveConfig) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(""));

        s.try_into()
    }
}
