use crate::dto::Message;
use crate::users::dto::{
    AccountID, 
    AccountResponse, 
    DiscordInitialResponse, 
    DiscordUserResponse, 
    GithubInitialResponse,
    GithubUserResponse, 
    OAuthCode, 
    SearchRequest, 
    UserCollectionsMessage, 
    UserCommentsMessage,
    UserPostsDTO, 
    UserPostsMessage, 
    SearchRequestMessage,
    UserExploreMessage
};
use actix_web::{
    get,
    web::{
        self, 
        Redirect
    },
    HttpRequest, 
    HttpResponse, 
    Responder,
};
use diesel::{
    ExpressionMethods, 
    QueryDsl, 
    QueryResult, 
    RunQueryDsl, 
    SelectableHelper
};
use pinned_db::create_connection;
use pinned_db_schema::schema::users::dsl::*;
use pinned_db_schema::{
    models::{
        Collection, 
        Comment, 
        NewUser, 
        Post, 
        User
    },
    schema::{
        collections, 
        comments, 
        posts, 
        users
    },
};
use pinned_utils::{
    get_discord_api_url, 
    get_env_var, 
    get_local_api_url, 
    iso8601
};
use rand::prelude::*;
use reqwest::StatusCode;
use sha2::{
    Digest, 
    Sha256
};
use std::{
    collections::HashMap, 
    time::SystemTime
};

// Authentication
#[get("/discord")]
pub async fn get_discord_user_authentication(
    data: web::Query<OAuthCode>,
) -> Result<Redirect, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut initial_code_request_data = HashMap::new();
    initial_code_request_data.insert("client_id", get_env_var("DISCORD_CLIENT_ID"));
    initial_code_request_data.insert("client_secret", get_env_var("DISCORD_CLIENT_SECRET"));
    initial_code_request_data.insert("code", data.code.to_owned());
    initial_code_request_data.insert("grant_type", "authorization_code".to_string());
    initial_code_request_data.insert("redirect_uri", get_local_api_url() + "/users/auth/discord");

    // Get authorization token and type
    let initial_response = client
        .post(format!("https://discord.com/api/oauth2/token"))
        .form(&initial_code_request_data)
        .header("content-type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    let initial_response_parsed: DiscordInitialResponse =
        serde_json::from_str(initial_response.text().await?.as_str())?;

    // Get user with auth token and type
    let user_response = client
        .get(format!("{}/users/@me", get_discord_api_url()))
        .header(
            "authorization",
            format!(
                "{} {}",
                initial_response_parsed.token_type, initial_response_parsed.access_token
            ),
        )
        .send()
        .await?;

    // Prevent parsing invalid data
    if user_response.status() != 200 {
        return Ok(Redirect::to(format!(
            "{}/user/login?msg=ue",
            get_env_var("FRONTEND_HOST")
        ))
        .permanent());
    }

    let user_response_parsed: DiscordUserResponse =
        serde_json::from_str(user_response.text().await?.as_str())?;

    let connection = &mut create_connection();

    let oauth: String = format!("discord-{}", user_response_parsed.id).to_string();
    let user: QueryResult<User> = users.filter(oauth_id.eq(oauth)).first::<User>(connection);

    // Check if a user already exists with OAuth provider
    if user.is_ok() {
        let user_unwrap = user.unwrap();

        let _ = diesel::update(users::table)
            .filter(users::id.eq(user_unwrap.id))
            .set((
                users::username.eq(user_response_parsed.global_name),
                users::avatar.eq(format!(
                    "https://cdn.discordapp.com/avatars/{}/{}",
                    user_response_parsed.id, user_response_parsed.avatar
                )),
            ))
            .execute(connection);
        return Ok(Redirect::to(format!(
            "{}/user/login?token={}",
            get_env_var("FRONTEND_HOST"),
            user_unwrap.token
        )));
    }

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut hasher = Sha256::new();
    hasher.update(
        format!(
            "{}{}",
            user_response_parsed.id,
            random_number * 2_000_000_000f64
        )
        .into_bytes(),
    );
    let user_token: String = format!("{:X}", hasher.finalize()).to_string();

    let new_user = NewUser {
        username: user_response_parsed.global_name,
        joined: iso8601(&SystemTime::now()),
        oauth_id: format!("discord-{}", user_response_parsed.id),
        avatar: format!(
            "https://cdn.discordapp.com/avatars/{}/{}",
            user_response_parsed.id, user_response_parsed.avatar
        ),
        bio: "No bio provided.".to_string(),
        token: user_token.clone(),
        collections: vec![],
    };

    let _ = diesel::insert_into(users::table)
        .values(new_user)
        .execute(connection)
        .expect("Failed to insert user");

    Ok(Redirect::to(format!(
        "{}/user/login?token={}",
        get_env_var("FRONTEND_HOST"),
        user_token
    ))
    .permanent())
}

#[get("/github")]
pub async fn get_github_user_authentication(
    data: web::Query<OAuthCode>,
) -> Result<Redirect, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    println!("{}", data.code.clone());
    let initial_token_response = client
        .post("https://github.com/login/oauth/access_token")
        .form(&[
            ("code", data.code.to_owned()),
            ("client_id", get_env_var("GITHUB_CLIENT_ID")),
            ("client_secret", get_env_var("GITHUB_CLIENT_SECRET")),
        ])
        .header("accept", "application/json")
        .send()
        .await?;
    let initial_response_parsed: GithubInitialResponse = serde_json::from_str::<GithubInitialResponse>(initial_token_response.text().await?.as_str())?;
    let user_response = client
        .get("https://api.github.com/user")
        .header("authorization", format!("{} {}", initial_response_parsed.token_type, initial_response_parsed.access_token))
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "request")
        .send()
        .await?;
    if user_response.status() != 200 {
        return Ok(Redirect::to(format!("{}/users/login?msg=ue", get_env_var("FRONTEND_URL"))).permanent());
    }
    let user_response_parsed: GithubUserResponse = serde_json::from_str::<GithubUserResponse>(user_response.text().await?.as_str())?;
    let oauth = format!("github-{}", user_response_parsed.id);
    let connection = &mut create_connection();
    let user: QueryResult<User> = users.filter(oauth_id.eq(oauth)).first::<User>(connection);

    // Check if a user already exists with OAuth provider
    if user.is_ok() {
        let user_unwrap = user.unwrap();

        let _ = diesel::update(users::table)
            .filter(users::id.eq(user_unwrap.id))
            .set((
                users::username.eq(user_response_parsed.login),
                users::avatar.eq(user_response_parsed.avatar_url),
            ))
            .execute(connection);

        return Ok(Redirect::to(format!(
            "{}/user/login?token={}",
            get_env_var("FRONTEND_HOST"),
            user_unwrap.token
        )));
    }

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut hasher = Sha256::new();
    hasher.update(
        format!(
            "{}{}",
            user_response_parsed.id,
            random_number * 2_000_000_000f64
        )
        .into_bytes(),
    );
    let user_token: String = format!("{:X}", hasher.finalize()).to_string();

    let new_user = NewUser {
        username: user_response_parsed.login,
        oauth_id: format!("github-{}", user_response_parsed.id),
        joined: iso8601(&SystemTime::now()),
        avatar: user_response_parsed.avatar_url,
        bio: "No bio provided.".to_string(),
        token: user_token.clone(),
        collections: vec![],
    };

    let _ = diesel::insert_into(users::table)
        .values(new_user)
        .execute(connection)
        .expect("Failed to insert user");

    Ok(Redirect::to(format!(
        "{}/user/login?token={}",
        get_env_var("FRONTEND_HOST"),
        user_token
    )))
}

// Information
#[get("")]
pub async fn get_account(
    request: HttpRequest,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let headers = request.headers();
    let user_token = headers.get("Authorization").unwrap().to_str();
    if user_token.is_err() {
        return Ok(HttpResponse::Ok()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid user token"));
    }

    let connection = &mut create_connection();
    let user: QueryResult<User> = users
        .filter(token.eq(user_token.unwrap()))
        .select(User::as_select())
        .first::<User>(connection);

    match user {
        Ok(u) => {
            let user_response = AccountResponse {
                message: "Fetched personal account".to_string(),
                user: u,
            };
            Ok(HttpResponse::Ok().json(user_response))
        }
        Err(e) => {
            let error_message = Message {
                message: e.to_string(),
            };
            Ok(HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .json(error_message))
        }
    }
}

#[get("/explore")]
pub async fn get_explore_users() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let users_result: QueryResult<Vec<User>> = users::table
        .select(User::as_select())
        .load(connection);
    if users_result.is_err() {
        return Ok(HttpResponse::Ok());
    }

    let max_return = 10;
    let all_users = users_result.expect("Failed to get users");

    if all_users.into_iter().count() <= max_return {
        let small_users: UserExploreMessage = UserExploreMessage {
            message: "Got users".to_string(),
            users: all_users
        };
        return Ok(HttpResponse::Ok().json(small_users));
    }

    let mut rng = thread_rng();
    let us: Vec<User> = all_users.into_iter().choose_multiple(&mut rng, max_return);

    let response_message: UserExploreMessage = UserExploreMessage {
        message: "Get users".to_string(),
        users: us
    };

    Ok(HttpResponse::Ok().json(response_message))
}

#[get("/public")]
pub async fn get_profile(
    data: web::Query<AccountID>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let user_id = data.id;
    if user_id <= 0 {
        let error_response = Message {
            message: "Failed to parse user ID".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::BAD_GATEWAY)
            .json(error_response));
    }

    let connection = &mut create_connection();
    let user: QueryResult<User> = users
        .find(user_id)
        .select(User::as_select())
        .first(connection);

    match user {
        Ok(mut user) => {
            user.token = "".to_string(); // TODO: better solution
            let user_response = AccountResponse {
                message: "Fetched public profile".to_string(),
                user,
            };
            Ok(HttpResponse::Ok().json(user_response))
        }
        Err(e) => {
            let error_message = Message {
                message: e.to_string(),
            };
            Ok(HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .json(error_message))
        }
    }
}

#[get("/posts")]
pub async fn get_users_posts(
    data: web::Query<UserPostsDTO>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();

    let user_result: QueryResult<User> = users::table
        .find(data.user_id)
        .select(User::as_select())
        .first::<User>(connection);

    if user_result.is_err() {
        let user_message = Message {
            message: "Failed to get user".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(user_message));
    }

    let user_unwrap = user_result.unwrap();
    let posts_result: QueryResult<Vec<Post>> = posts::table
        .filter(posts::creator.eq(user_unwrap.id))
        .select(Post::as_select())
        .load(connection);

    if posts_result.is_err() {
        let posts_message = Message {
            message: "Failed to get posts".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(posts_message));
    }

    let posts_unwrap = posts_result.unwrap();

    let success_message = UserPostsMessage {
        message: "Fetched user's posts".to_string(),
        posts: posts_unwrap,
    };
    Ok(HttpResponse::Ok().json(success_message))
}

#[get("/collections")]
pub async fn get_user_collections(
    data: web::Query<UserPostsDTO>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();

    let user_result: QueryResult<User> = users::table
        .find(data.user_id)
        .select(User::as_select())
        .first::<User>(connection);

    if user_result.is_err() {
        let user_message = Message {
            message: "Failed to get user".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(user_message));
    }

    let user_unwrap = user_result.unwrap();
    let collections_result: QueryResult<Vec<Collection>> = collections::table
        .filter(collections::creator.eq(user_unwrap.id))
        .select(Collection::as_select())
        .load(connection);

    if collections_result.is_err() {
        let collections_message = Message {
            message: "Failed to get posts".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(collections_message));
    }

    let collections_unwrap = collections_result.unwrap();

    let success_message = UserCollectionsMessage {
        message: "Fetched user's collections".to_string(),
        collections: collections_unwrap,
    };
    Ok(HttpResponse::Ok().json(success_message))
}

#[get("/comments")]
pub async fn get_users_comments(
    data: web::Query<UserPostsDTO>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();

    let user_result: QueryResult<User> = users::table
        .find(data.user_id)
        .select(User::as_select())
        .first::<User>(connection);

    if user_result.is_err() {
        let user_message = Message {
            message: "Failed to get user".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(user_message));
    }

    let user_unwrap = user_result.unwrap();
    let comments_result: QueryResult<Vec<Comment>> = comments::table
        .filter(comments::creator.eq(user_unwrap.id))
        .select(Comment::as_select())
        .load(connection);

    if comments_result.is_err() {
        let comments_message = Message {
            message: "Failed to get comments".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::NOT_FOUND)
            .json(comments_message));
    }

    let comments_unwrap = comments_result.unwrap();

    let success_message = UserCommentsMessage {
        message: "Fetched user's comments".to_string(),
        comments: comments_unwrap,
    };
    Ok(HttpResponse::Ok().json(success_message))
}

#[get("/search")]
pub async fn get_search_users(
    data: web::Query<SearchRequest>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let mut users_vec: Vec<User> = Vec::new();

    if data.id != 0 {
        let user_result: QueryResult<User> = users::table
            .find(data.id)
            .first::<User>(connection);
        if user_result.is_ok() {
            let user: User = user_result.unwrap();
            users_vec.push(user);
        }
    }

    let user_results: QueryResult<Vec<User>> = users::table
        .load(connection);

    if user_results.is_ok() {
        let users_unwrap: Vec<User> = user_results.unwrap();
        let mut index = 0;
        for user in users_unwrap {
            if index > 15 {
                break;
            }
            if user.username.contains(data.username.as_str()) {
                users_vec.push(user);
            }
            index += 1;
        }
    }

    Ok(HttpResponse::Ok().json(SearchRequestMessage { message: "Fetched users".to_string(), users: users_vec }))
}
