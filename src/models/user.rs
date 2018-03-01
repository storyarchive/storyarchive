use std::string::String;

use bson::Document;

use models::story::Story;

pub struct User {
    pub authors_favorite: Vec<User>,
    pub authors_following: Vec<User>,
    pub description: String,
    pub email: String,
    pub rank: u32,
    // Figure out how to use MongoDB ObjecID
    pub stories: Vec<Story>,
    pub stories_favorite: Vec<Story>,
    pub stories_following: Vec<Story>,
    pub username: String,
}

impl User {
    pub fn create_user_doc(self) -> Document {
        doc! {

        }
    }
}