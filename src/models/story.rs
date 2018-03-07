use std::collections::HashMap;
use std::string::String;

use bson::Document;
use bson::oid::ObjectId;

/// The story and its information.
///
/// # Fields
///   * ```chapters```: A hashmap containing each chapter by its ChapterId and its MongoDB ObjectId
///     (see ```Chapter``` struct)
///   * ```meta```: Contains overall information on the story and its chapters
///     (see ```StoryMeta``` struct)
#[derive(Clone, Deserialize)]
pub struct Story {
    pub chapters: Option<HashMap<u32, ObjectId>>,
    pub meta: StoryMeta,
}

/// The story's overall information.
///
/// # Fields
///   * ```favorites```: Stores the story's favorite count.
///   * ```follows```: Stores the story's follow count.
///   * ```views```: Stores the story's view count.
///   * ```words```: Stores the story's total word count.
#[derive(Clone, Deserialize)]
pub struct StoryMeta {
    pub favorites: u32,
    pub follows: u32,
    pub views: u32,
    pub words: u32,
}

/// The story's tags
/// # Fields
#[derive(Clone, Deserialize)]
pub struct StoryTags {

}

/// The chapter and its information.
///
/// # Fields
///   * ```content```: Contains the main body of a chapter
///     (see ```ChapterContent``` struct)
///   * ```meta```: Contains overall information on the chapter
///     (see ```ChapterMeta``` struct)
#[derive(Clone, Deserialize)]
pub struct Chapter {
    pub content: ChapterContent,
    pub meta: ChapterMeta,
}

/// The raw and rendered forms of the chapter's markdown
/// # Fields
///   * ```raw```: Contains the raw markdown of the chapter
///     (see ```ChapterContentRaw``` struct)
///   * ```rendered```: Contains the rendered markdown of the chapter
///     (see ```ChapterContentRendered``` struct)
#[derive(Clone, Deserialize)]
pub struct ChapterContent {
    pub raw: ChapterContentRaw,
    pub rendered: ChapterContentRendered,
}

/// The chapter's raw non-rendered markdown
/// # Fields
#[derive(Clone, Deserialize)]
pub struct ChapterContentRaw {
    pub authors_note: String,
    pub body: String,
}

/// The chapter's rendered markdown
/// # Fields
#[derive(Clone, Deserialize)]
pub struct ChapterContentRendered {
    pub authors_note: String,
    pub body: String,
}

/// The overall information of the chapter
/// # Fields
///   * ```words```: Stores the chapter's total word count
#[derive(Clone, Deserialize)]
pub struct ChapterMeta {
    pub words: u32,
}