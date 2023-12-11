use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct GetCommentDTO {
    pub comment_id: i32
}

#[derive(Deserialize)]
pub struct NewCommentDTO {
    pub content: String,
    pub post: i32
}

#[derive(Serialize)]
pub struct NewCommentMessage {
    pub message: String,
    pub comment_id: usize 
}
