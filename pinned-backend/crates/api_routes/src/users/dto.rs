use pinned_db_schema::models::{User, Post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OAuthCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct DiscordInitialResponse {
    pub access_token: String,
    pub token_type: String
}

#[derive(Deserialize)]
pub struct DiscordUserResponse {
    pub global_name: String,
    pub avatar: String,
    pub id: String 
}

#[derive(Deserialize)]
pub struct GithubInitialResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String
}

#[derive(Deserialize)]
pub struct GithubUserResponse {
    pub login: String,
    pub avatar_url: String,
    pub id: u64
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub message: String,
    pub token: String
}

#[derive(Deserialize)]
pub struct AccountID {
    pub id: i32
}

#[derive(Serialize, Deserialize)]
pub struct AccountResponse {
    pub message: String,
    pub user: User
}

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub username: String,
    pub id: i32
}

#[derive(Deserialize)]
pub struct UserPostsDTO {
    pub user_id: i32
}

#[derive(Serialize)]
pub struct UserPostsMessage {
    pub message: String,
    pub posts: Vec<Post>
}

#[derive(Serialize)]
pub struct UserSearchResponse {
    pub users: Vec<User>
}
