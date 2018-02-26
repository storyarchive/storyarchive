use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::string::String;

use config::storyarchive::StoryArchiveConfig;
use config::theme::ThemeConfig;

use sass_rs::{Options, compile_file};

#[derive(Clone)]
pub struct Sass {
    pub compiled: String,
    storyarchive_config: StoryArchiveConfig,
    theme_config: ThemeConfig,
}

impl Sass {
    // TODO: Get this to work with SASS AND SCSS
    pub fn new(
        storyarchive_config: StoryArchiveConfig,
        theme_config: ThemeConfig,
    ) -> Result<Sass, String> {
        let compiled = compile_file(
            format!(
                "./{}/{}/{}/_scss/{}.scss",
                storyarchive_config.general.themes_dir.clone(),
                storyarchive_config.general.theme.clone(),
                theme_config.theme.assets.clone(),
                theme_config.sass.input.clone()
            ),
            Options::default(),
        )?;

        Ok(Sass {
            compiled,
            storyarchive_config: storyarchive_config.clone(),
            theme_config: theme_config.clone()
        })
    }

    pub fn write(self) -> Result<Self, String> {
        let mut file = File::create(Path::new(
            &format!(
                "./{}/{}/{}/{}.css",
                self.storyarchive_config.general.themes_dir.clone(),
                self.storyarchive_config.general.theme.clone(),
                self.theme_config.theme.assets.clone(),
                self.theme_config.sass.output.clone()
            )
        )).map_err(|e| e.to_string())?;

        file.write_all(self.compiled.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(self)
    }
}