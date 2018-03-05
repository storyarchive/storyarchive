use std::string::String;

use bson::Document;
use bson::oid::ObjectId;

use models::story::Story;

/// A user and its information and settings
///
/// # Fields
///   * ```authors_favorite```: A vector of ObjectIds of the user's favorite users
///   * ```authors_following```: A vector of ObjectIds of the users that the user is following
///   * ```description```: The biography of the user
///   * ```email```: The user's email address
///   * ```favorite```: A vector of ObjectIds of users that are favorites of the user
///   * ```followers```: A vector of ObjectIds of users that follow the user
///   * ```rank```: The users rank
///   * ```stories```: A vector of ObjectIds of stories that the user has
///   * ```stories_favorite```: A vector of ObjectIds of stories that the user favorites
///   * ```stories_following```: A vector of ObjectIds of stories that the user follows
///   * ```username```: The visible username of the user
pub struct User {
    pub authors_favorite: Vec<ObjectId>,
    pub authors_following: Vec<ObjectId>,
    pub description: String,
    pub email: String,
    pub favorite: Vec<ObjectId>,
    pub followers: Vec<ObjectId>,
    pub rank: u32,
    pub stories: Vec<ObjectId>,
    pub stories_favorite: Vec<ObjectId>,
    pub stories_following: Vec<ObjectId>,
    pub username: String,
}

impl User {
    /// Transforms the user struct into a MongoDB Bson OrderedDocument
    ///
    /// # Example
    /// ```rust
    /// let user = User...;
    /// mongodb_collection.insert_one(user.create_user_doc().clone(), None);
    /// ```
    pub fn create_user_doc(self) -> Document {
        doc! {

        }
    }
}
