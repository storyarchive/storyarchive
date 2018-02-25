use std::collections::HashMap;
use std::string::String;

pub struct Story {
    pub chapters: HashMap<u16, Chapter>, // 65535
    pub meta: StoryMeta,
}

pub struct StoryMeta {
    pub favorites: u32, // 4294967295
    pub follows: u32,
    pub views: u32,
    pub words: u32,
}

pub struct Chapter {
    pub meta: ChapterMeta,
}

pub struct ChapterContent {
    pub authors_note: String,
    pub body: String,
}

pub struct ChapterMeta {
    pub words: u32,
}