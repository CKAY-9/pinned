use std::collections::HashMap;
use pinned_api_routes::create_connection;
use pinned_utils::{get_env_var, get_discord_api_url, get_local_api_url};
use pinned_db_schema::{schema::{self, users}, models::{NewUser, User}};
use actix_web::{get, Responder, HttpResponse, web, HttpRequest};
use serde::{Deserialize, Serialize};

// Authentication

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
struct TokenResponse {
    message: String,
    token: String
}

#[get("/discord")]
pub async fn discord_user_authentication(data: web::Query<OAuthCode>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut initial_code_request_data = HashMap::new();
    initial_code_request_data.insert("client_id", get_env_var("DISCORD_CLIENT_ID"));
    initial_code_request_data.insert("client_secret", get_env_var("DISCORD_CLIENT_SECRET"));
    initial_code_request_data.insert("code", data.code.to_string());
    initial_code_request_data.insert("grant_type", "authorization_code".to_string());
    initial_code_request_data.insert("redirect_uri", get_local_api_url() + "/users/discord"); 

    let inital_response = client.post(format!("{}/oauth2/token", get_discord_api_url()))
        .form(&initial_code_request_data)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    let inital_response_parsed: DiscordInitialResponse = serde_json::from_str(inital_response.text().await?.as_str())?;

    let user_response = client.get(format!("{}/users/@me", get_discord_api_url()))
        .header("authorization", format!("{} {}", inital_response_parsed.token_type, inital_response_parsed.access_token))
        .send()
        .await?;
    let user_response_parsed: DiscordUserResponse = serde_json::from_str(user_response.text().await?.as_str())?;

    let new_user = NewUser {
        username: user_response_parsed.global_name,
        avatar: format!("https://cdn.discordapp.com/avatars/{}/{}", user_response_parsed.id, user_response_parsed.avatar),
        bio: "No bio provided.".to_string(),
        token: "".to_string()
    };

    let connection = create_connection();
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection);

    let response: TokenResponse = TokenResponse { message: "Logged in with Discord".to_string(), token: "test".to_string() };

    Ok(HttpResponse::Ok().json(&response))
}

#[get("/github")]
pub async fn github_user_authentication(data: web::Query<OAuthCode>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let initial_token_response = client.post("https://github.com/login/oauth/access_token")
        .form(&[
            ("code", data.code.to_owned()),
            ("client_id", get_env_var("GITHUB_CLIENT_ID")),
            ("client_secret", get_env_var("GITHUB_CLIENT_SECRET"))
        ])
        .header("accept", "application/json")
        .send()
        .await?;
    let initial_response_parsed: GithubInitialResponse = serde_json::from_str(initial_token_response.text().await?.as_str())?;

    let user_response = client.get("https://api.github.com/user")
        .header("authorization", format!("{} {}", initial_response_parsed.token_type, initial_response_parsed.access_token))
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "request")
        .send()
        .await?;
    let user_response_parsed: GithubUserResponse = serde_json::from_str(user_response.text().await?.as_str())?;

    let response: TokenResponse = TokenResponse { message: "Logged in with GitHub".to_string(), token: "test".to_string() };

    Ok(HttpResponse::Ok().json(&response))
}

// Information
#[derive(Deserialize)]
pub struct AccountID {
    pub account_id: usize 
}

#[get("/")]
pub async fn get_account(request: HttpRequest, data: web::Query<AccountID>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = request.headers().get("Authorization").unwrap();
    let id = data.account_id.to_owned();

    Ok(HttpResponse::Ok())
}
