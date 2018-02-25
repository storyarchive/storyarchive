use std::collections::HashMap;

use rocket::{Route, State};
use rocket_contrib::Template;

use config::storyarchive::StoryArchiveConfig;
use config::theme::ThemeConfig;
use extras::sass::Sass;

pub fn routes() -> Vec<Route> {
    routes![
        home,
        login, logout, register
    ]
}

#[get("/")]
pub fn home(
    _storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
    sass: State<Sass>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");
    context.insert("css", &sass.compiled);

    Template::render(
        format!(
            "{}",
            theme_config.path.hub.home
        ),
        &context,
    )
}

#[get("/login")]
pub fn login(
    _storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
    sass: State<Sass>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");
    context.insert("css", &sass.compiled);

    Template::render(
        format!(
            "{}",
            theme_config.path.hub.login
        ),
        &context,
    )
}

#[get("/logout")]
pub fn logout(
    _storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
    sass: State<Sass>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");
    context.insert("css", &sass.compiled);

    Template::render(
        format!(
            "{}",
            theme_config.path.hub.home
        ),
        &context,
    )
}

#[get("/register")]
pub fn register(
    _storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
    sass: State<Sass>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");
    context.insert("css", &sass.compiled);

    Template::render(
        format!(
            "{}",
            theme_config.path.hub.register
        ),
        &context,
    )
}
