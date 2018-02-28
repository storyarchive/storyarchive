use std::collections::HashMap;

use rocket::{Route, State};
use rocket_contrib::Template;

use config::storyarchive::StoryArchiveConfig;
use config::theme::ThemeConfig;
use extras::sass::Sass;
use models::page::Page;
use models::page::hub::{Hub};

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
    let context = Page {
        css: &sass.compiled,
        hub: None,
    };

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
    let context = Page {
        css: &sass.compiled,
        hub: None,
    };

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
    let context = Page {
        css: &sass.compiled,
        hub: None,
    };

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
    let context = Page {
        css: &sass.compiled,
        hub: None,
    };

    Template::render(
        format!(
            "{}",
            theme_config.path.hub.register
        ),
        &context,
    )
}
