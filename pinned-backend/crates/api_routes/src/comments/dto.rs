use pinned_db_schema::models::Comment;
use serde::{
    Deserialize, 
    Serialize
};

#[derive(Deserialize)]
pub struct GetCommentDTO {
    pub comment_id: i32,
}

#[derive(Serialize)]
pub struct GetCommentMessage {
    pub message: String,
    pub comment: Comment,
}

#[derive(Deserialize)]
pub struct NewCommentDTO {
    pub content: String,
    pub post: i32,
}

#[derive(Serialize)]
pub struct NewCommentMessage {
    pub message: String,
    pub comment_id: i32,
}

#[derive(Deserialize)]
pub struct LikeCommentDTO {
    pub comment_id: i32,
    pub like_type: i32
}
