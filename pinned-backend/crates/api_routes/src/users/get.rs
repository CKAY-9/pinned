use std::{collections::HashMap, time::SystemTime};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, QueryResult, SelectableHelper};
use pinned_db::create_connection;
use pinned_utils::{get_env_var, get_discord_api_url, get_local_api_url, iso8601};
use pinned_db_schema::{schema::users, models::{NewUser, User}};
use actix_web::{get, Responder, HttpResponse, web::{self, Redirect}, HttpRequest};
use reqwest::StatusCode;
use sha2::{Sha256, Digest};
use pinned_db_schema::schema::users::dsl::*;
use rand::prelude::*;
use crate::dto::Message;
use crate::users::dto::{
    DiscordUserResponse,
    DiscordInitialResponse,
    GithubUserResponse,
    GithubInitialResponse,
    AccountID,
    AccountResponse,
    OAuthCode,
    SearchRequest,
    UserSearchResponse
};

// Authentication
#[get("/discord")]
pub async fn get_discord_user_authentication(data: web::Query<OAuthCode>) -> Result<Redirect, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut initial_code_request_data = HashMap::new();
    initial_code_request_data.insert("client_id", get_env_var("DISCORD_CLIENT_ID"));
    initial_code_request_data.insert("client_secret", get_env_var("DISCORD_CLIENT_SECRET"));
    initial_code_request_data.insert("code", data.code.to_string());
    initial_code_request_data.insert("grant_type", "authorization_code".to_string());
    initial_code_request_data.insert("redirect_uri", get_local_api_url() + "/users/auth/discord"); 

    // Get authorization token and type
    let inital_response = client.post(format!("{}/oauth2/token", get_discord_api_url()))
        .form(&initial_code_request_data)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    let inital_response_parsed: DiscordInitialResponse = serde_json::from_str(inital_response.text().await?.as_str())?;

    // Get user with auth token and type
    let user_response = client.get(format!("{}/users/@me", get_discord_api_url()))
        .header("authorization", format!("{} {}", inital_response_parsed.token_type, inital_response_parsed.access_token))
        .send()
        .await?;

    // Prevent parsing invalid data
    if user_response.status() != 200 {
        return Ok(Redirect::to(format!("{}/user/login?msg=ue", get_env_var("FRONTEND_HOST"))).permanent());
    }

    let user_response_parsed: DiscordUserResponse = serde_json::from_str(user_response.text().await?.as_str())?;

    let connection = &mut create_connection();

    let oauth: String = format!("discord-{}", user_response_parsed.id).to_string();
    let user: QueryResult<User> = users.filter(oauth_id.eq(oauth)).first::<User>(connection);

    // Check if a user already exists with OAuth provider
    if user.is_ok() {
        // TODO: Update user
        return Ok(Redirect::to(format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user.unwrap().token)));
    }

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", user_response_parsed.id, random_number * 2_000_000_000f64).into_bytes());
    let user_token: String = format!("{:X}", hasher.finalize()).to_string();

    let new_user = NewUser {
        username: user_response_parsed.global_name,
        joined: iso8601(&SystemTime::now()),
        oauth_id: format!("discord-{}", user_response_parsed.id),
        avatar: format!("https://cdn.discordapp.com/avatars/{}/{}", user_response_parsed.id, user_response_parsed.avatar),
        bio: "No bio provided.".to_string(),
        token: user_token.clone(),
        collections: vec![]
    };

    let _ = diesel::insert_into(users::table)
        .values(new_user)
        .execute(connection)
        .expect("Failed to insert user");

    Ok(Redirect::to(format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user_token)).permanent())
}

#[get("/github")]
pub async fn get_github_user_authentication(data: web::Query<OAuthCode>) -> Result<Redirect, Box<dyn std::error::Error>> {
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
    let oauth = format!("gituhb-{}", user_response_parsed.id);
    let connection = &mut create_connection();
    let user: QueryResult<User> = users.filter(oauth_id.eq(oauth)).first::<User>(connection);
    // Check if a user already exists with OAuth provider
    if user.is_ok() {
        // TODO: Update user
        return Ok(Redirect::to(format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user.unwrap().token)));
    }

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", user_response_parsed.id, random_number * 2_000_000_000f64).into_bytes());
    let user_token: String = format!("{:X}", hasher.finalize()).to_string();

    let new_user = NewUser {
        username: user_response_parsed.login,
        oauth_id: format!("discord-{}", user_response_parsed.id),
        joined: iso8601(&SystemTime::now()),
        avatar: user_response_parsed.avatar_url,
        bio: "No bio provided.".to_string(),
        token: user_token.clone(),
        collections: vec![] 
    };

    let _ = diesel::insert_into(users::table)
        .values(new_user)
        .execute(connection)
        .expect("Failed to insert user");

    Ok(Redirect::to(format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user_token)))
}

// Information
#[get("")]
pub async fn get_account(request: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let headers = request.headers();
    let user_token = headers.get("Authorization").unwrap().to_str();
    if user_token.is_err() {
        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).body("Invalid user token"));
    }

    let connection = &mut create_connection();
    let user: QueryResult<User> = users.filter(token.eq(user_token.unwrap()))
        .select(User::as_select())
        .first::<User>(connection);

    match user {
        Ok(u) => {
            let user_response = AccountResponse { message: "Fetched personal account".to_string(), user: u };
            Ok(HttpResponse::Ok().json(user_response))
        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(error_message))
        }
    }
}

#[get("/public")]
pub async fn get_profile(data: web::Query<AccountID>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let user_id = data.id;
    if user_id <= 0 {
        let error_response = Message { message: "Failed to parse user ID".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_GATEWAY).json(error_response));
    }

    let connection = &mut create_connection();
    let user: QueryResult<User> = users.find(user_id)
        .select(User::as_select()) 
        .first(connection);

    match user {
        Ok(mut user) => {
            user.token = "".to_string(); // TODO: better solution
            let user_response = AccountResponse { message: "Fetched public profile".to_string(), user };
            Ok(HttpResponse::Ok().json(user_response))
        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(error_message))
        }
    }
}

#[get("/search")]
pub async fn get_search_users(data: web::Query<SearchRequest>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let name: String = data.username.clone();

    let connection = &mut create_connection();
    let results: QueryResult<Vec<User>> = users::table
        .filter(users::username.eq(name))
        .limit(10)
        .select(User::as_select())
        .load(connection);

    if results.is_err() {
        let results_error_message = Message { message: "Failed to complete search".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(results_error_message));
    }

    let response = UserSearchResponse { users: results.unwrap() };
    Ok(HttpResponse::Ok().json(response))
}
