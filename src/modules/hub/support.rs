use rocket::State;
use rocket_contrib::Template;

use config::storyarchive::StoryArchiveConfig;
use config::theme::ThemeConfig;
use extras::sass::Sass;
use models::page::Page;
use models::page::hub::{Hub};

#[get("/support")]
pub fn support(
    _storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
    sass: State<Sass>,
) -> Template {
    let context = Page {
        css: &sass.compiled,
        hub: None,
    };

    Template::render(
        format!(
            "{}",
            theme_config.path.hub.support,
        ),
        &context,
    )
}