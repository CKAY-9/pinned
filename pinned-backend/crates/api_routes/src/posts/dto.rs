use pinned_db_schema::models::Post;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewPostDTO {
    pub title: String,
    pub file_id: String,
    pub content: String
}

#[derive(Serialize)]
pub struct NewPostOTD {
    pub message: String,
    pub post_id: usize 
}

#[derive(Deserialize)]
pub struct GetPostDTO {
    pub post_id: i32
}

#[derive(Serialize)]
pub struct GetPostMessage {
    pub message: String,
    pub post: Post
}
