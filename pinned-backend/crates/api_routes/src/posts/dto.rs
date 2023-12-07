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
