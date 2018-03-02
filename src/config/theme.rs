use cconfig::{Config, ConfigError, File};

use helpers;

#[derive(Clone, Deserialize)]
pub struct Theme {
    pub assets: String,
    pub rest: bool,
}

#[derive(Clone, Deserialize)]
pub struct Fluent {
    pub languages: PathHub,
}

#[derive(Clone, Deserialize)]
pub struct FluentLanguages {
    pub languages: PathHub,
}

#[derive(Clone, Deserialize)]
pub struct Path {
    pub hub: PathHub,
}

#[derive(Clone, Deserialize)]
pub struct PathHub {
    pub home: String,
    pub login: String,
    pub register: String,
}


#[derive(Clone, Deserialize)]
pub struct Sass {
    pub input: String,
    pub output: String,
}

#[derive(Clone, Deserialize)]
pub struct ThemeConfig {
    pub theme: Theme,
    pub path: Path,
    pub sass: Sass,
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