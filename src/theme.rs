use cconfig::{Config, ConfigError, File};

use config;
use helpers;

pub struct Hub {

}

#[derive(Deserialize)]
pub struct Theme {
    pub rest: Some(bool),
}

#[derive(Deserialize)]
pub struct ThemeConfig {
    pub theme: Theme,
}

pub enum Pages {
    HubHome,
}

impl ThemeConfig {
    pub fn new(config: &config::StoryArchiveConfig) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(""));

        s.try_into()
    }
}
