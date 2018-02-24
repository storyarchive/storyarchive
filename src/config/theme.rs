use cconfig::{Config, ConfigError, File};

use helpers;

#[derive(Deserialize)]
pub struct Theme {
    pub assets: String,
    pub rest: bool,
}

#[derive(Deserialize)]
pub struct PathHub {
    pub home: String,
    pub login: String,
    pub register: String,
}

#[derive(Deserialize)]
pub struct Path {
    pub hub: PathHub,
}

#[derive(Deserialize)]
pub struct ThemeConfig {
    pub theme: Theme,
    pub path: Path,
}

impl ThemeConfig {
    pub fn new(theme: String, themes_dir: String) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(
            helpers::join(&[themes_dir, theme, "Theme".to_owned()]).to_str()
                .expect("Unable to form path [theme config]")
        ))?;

        s.try_into()
    }
}