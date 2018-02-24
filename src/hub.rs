use std::collections::HashMap;

use rocket::{Route, State};
use rocket_contrib::Template;

use config::storyarchive::StoryArchiveConfig;
use config::theme::ThemeConfig;

pub fn routes() -> Vec<Route> {
    routes![
        home,
        login, logout, register
    ]
}

#[get("/")]
pub fn home(
    storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");

    Template::render(
        format!(
            "{}/{}/{}",
            storyarchive_config.general.themes_dir,
            storyarchive_config.general.theme,
            theme_config.path.hub.home
        ),
        &context
    )
}

#[get("/login")]
pub fn login(
    storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");

    Template::render(
        format!(
            "{}/{}/{}",
            storyarchive_config.general.themes_dir,
            storyarchive_config.general.theme,
            theme_config.path.hub.login
        ),
        &context
    )
}

#[get("/logout")]
pub fn logout(
    storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");

    Template::render(
        format!(
            "{}/{}/{}",
            storyarchive_config.general.themes_dir,
            storyarchive_config.general.theme,
            theme_config.path.hub.home
        ),
        &context
    )
}

#[get("/register")]
pub fn register(
    storyarchive_config: State<StoryArchiveConfig>,
    theme_config: State<ThemeConfig>,
) -> Template {
    let mut context = HashMap::new();
    context.insert("path", "");

    Template::render(
        format!(
            "{}/{}/{}",
            storyarchive_config.general.themes_dir,
            storyarchive_config.general.theme,
            theme_config.path.hub.register
        ),
        &context
    )
}
