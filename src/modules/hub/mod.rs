pub mod home;
pub mod login;
pub mod logout;
pub mod register;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        home::home,
        login::login, logout::logout, register::register,
    ]
}
