use bson::Document;

pub struct User {
    pub rank: u8, // 0-255
}

impl User {
    pub fn create_user_doc(self) -> Document {
        doc! {

        }
    }
}