use std::vec::Vec;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![home]
}

#[get("/")]
pub fn home() -> &'static str {
    "Hello from Hub"
}
