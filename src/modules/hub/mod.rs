pub mod about;
pub mod home;
pub mod login;
pub mod logout;
pub mod privacy;
pub mod register;
pub mod support;
pub mod terms;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        home::home,
        about::about,
        login::login, logout::logout, register::register,
        privacy::privacy,
        support::support,
        terms::terms,
    ]
}
