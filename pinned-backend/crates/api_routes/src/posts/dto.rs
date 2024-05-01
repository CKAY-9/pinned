use pinned_db_schema::models::Post;
use serde::{
    Deserialize, 
    Serialize
};

#[derive(Deserialize)]
pub struct NewPostDTO {
    pub title: String,
    pub file_id: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct PostExploreMessage {
    pub message: String,
    pub posts: Vec<Post>
}

#[derive(Deserialize)]
pub struct UpdatePostDTO {
    pub title: String,
    pub description: String,
    pub post_id: i32,
}

#[derive(Deserialize)]
pub struct SearchFilterDTO {

}

#[derive(Serialize)]
pub struct NewPostOTD {
    pub message: String,
    pub post_id: i32,
}

#[derive(Deserialize)]
pub struct GetPostDTO {
    pub post_id: i32,
}

#[derive(Serialize)]
pub struct GetPostMessage {
    pub message: String,
    pub post: Post,
}

#[derive(Deserialize)]
pub struct LikePostDTO {
    pub like_type: i32,
    pub post_id: i32
}

#[derive(Deserialize)]
pub struct SearchPostsDTO {
    pub name: String,
    pub post_id: i32
}

#[derive(Serialize)]
pub struct SearchPostsMessages {
    pub message: String,
    pub posts: Vec<Post>
}
