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
    UserExploreMessage,
};
use actix_web::{ get, web::{ self, Redirect }, HttpRequest, HttpResponse, Responder };
use diesel::{ ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper };
use pinned_db::create_connection;
use pinned_db::crud::users::{
    get_user_collections_from_id,
    get_user_comments_from_id,
    get_user_from_id,
    get_user_from_token,
    get_user_posts_from_id,
};
use pinned_db_schema::schema::users::dsl::*;
use pinned_db_schema::{ models::{ NewUser, User }, schema::users };
use pinned_utils::{
    extract_header_value,
    get_discord_api_url,
    get_env_var,
    get_local_api_url,
    iso8601,
};
use rand::prelude::*;
use reqwest::StatusCode;
use sha2::{ Digest, Sha256 };
use std::{ collections::HashMap, time::SystemTime };

// Authentication
#[get("/discord")]
pub async fn get_discord_user_authentication(
    data: web::Query<OAuthCode>
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
        .send().await?;
    let initial_response_parsed = serde_json::from_str::<DiscordInitialResponse>(
        initial_response.text().await?.as_str()
    )?;

    // Get user with auth token and type
    let user_response = client
        .get(format!("{}/users/@me", get_discord_api_url()))
        .header(
            "authorization",
            format!(
                "{} {}",
                initial_response_parsed.token_type,
                initial_response_parsed.access_token
            )
        )
        .send().await?;

    // Prevent parsing invalid data
    if user_response.status() != 200 {
        return Ok(
            Redirect::to(format!("{}/user/login?msg=ue", get_env_var("FRONTEND_HOST"))).permanent()
        );
    }

    let user_response_parsed: DiscordUserResponse = serde_json::from_str(
        user_response.text().await?.as_str()
    )?;

    let connection = &mut create_connection();

    let oauth: String = format!("discord-{}", user_response_parsed.id).to_string();
    let user: QueryResult<User> = users.filter(oauth_id.eq(oauth)).first::<User>(connection);

    // Check if a user already exists with OAuth provider
    if user.is_ok() {
        let user_unwrap = user.unwrap();

        let _ = diesel
            ::update(users::table)
            .filter(users::id.eq(user_unwrap.id))
            .set((
                users::username.eq(user_response_parsed.global_name),
                users::avatar.eq(
                    format!(
                        "https://cdn.discordapp.com/avatars/{}/{}",
                        user_response_parsed.id,
                        user_response_parsed.avatar
                    )
                ),
            ))
            .execute(connection);
        return Ok(
            Redirect::to(
                format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user_unwrap.token)
            )
        );
    }

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut hasher = Sha256::new();
    hasher.update(
        format!("{}{}", user_response_parsed.id, random_number * 2_000_000_000f64).into_bytes()
    );
    let user_token: String = format!("{:X}", hasher.finalize()).to_string();

    let new_user = NewUser {
        username: user_response_parsed.global_name,
        joined: iso8601(&SystemTime::now()),
        oauth_id: format!("discord-{}", user_response_parsed.id),
        avatar: format!(
            "https://cdn.discordapp.com/avatars/{}/{}",
            user_response_parsed.id,
            user_response_parsed.avatar
        ),
        bio: "No bio provided.".to_string(),
        token: user_token.clone(),
        collections: vec![],
        favourites: vec![],
        pinned: vec![],
    };

    let _ = diesel
        ::insert_into(users::table)
        .values(new_user)
        .execute(connection)
        .expect("Failed to insert user");

    Ok(
        Redirect::to(
            format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user_token)
        ).permanent()
    )
}

#[get("/github")]
pub async fn get_github_user_authentication(
    data: web::Query<OAuthCode>
) -> Result<Redirect, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    println!("{}", data.code.clone());
    let initial_token_response = client
        .post("https://github.com/login/oauth/access_token")
        .form(
            &[
                ("code", data.code.to_owned()),
                ("client_id", get_env_var("GITHUB_CLIENT_ID")),
                ("client_secret", get_env_var("GITHUB_CLIENT_SECRET")),
            ]
        )
        .header("accept", "application/json")
        .send().await?;
    let initial_response_parsed: GithubInitialResponse =
        serde_json::from_str::<GithubInitialResponse>(
            initial_token_response.text().await?.as_str()
        )?;
    let user_response = client
        .get("https://api.github.com/user")
        .header(
            "authorization",
            format!(
                "{} {}",
                initial_response_parsed.token_type,
                initial_response_parsed.access_token
            )
        )
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "request")
        .send().await?;
    if user_response.status() != 200 {
        return Ok(
            Redirect::to(format!("{}/users/login?msg=ue", get_env_var("FRONTEND_URL"))).permanent()
        );
    }
    let user_response_parsed: GithubUserResponse = serde_json::from_str::<GithubUserResponse>(
        user_response.text().await?.as_str()
    )?;
    let oauth = format!("github-{}", user_response_parsed.id);
    let connection = &mut create_connection();
    let user: QueryResult<User> = users.filter(oauth_id.eq(oauth)).first::<User>(connection);

    // Check if a user already exists with OAuth provider
    if user.is_ok() {
        let user_unwrap = user.unwrap();

        let _ = diesel
            ::update(users::table)
            .filter(users::id.eq(user_unwrap.id))
            .set((
                users::username.eq(user_response_parsed.login),
                users::avatar.eq(user_response_parsed.avatar_url),
            ))
            .execute(connection);

        return Ok(
            Redirect::to(
                format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user_unwrap.token)
            )
        );
    }

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let mut hasher = Sha256::new();
    hasher.update(
        format!("{}{}", user_response_parsed.id, random_number * 2_000_000_000f64).into_bytes()
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
        favourites: vec![],
        pinned: vec![],
    };

    let _ = diesel
        ::insert_into(users::table)
        .values(new_user)
        .execute(connection)
        .expect("Failed to insert user");

    Ok(Redirect::to(format!("{}/user/login?token={}", get_env_var("FRONTEND_HOST"), user_token)))
}

// Information
#[get("")]
pub async fn get_account(
    request: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let u_token = extract_header_value(&request, "Authorization");
    if u_token.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user token".to_string(),
            })
        );
    }

    let user_option = get_user_from_token(u_token.unwrap());
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    Ok(
        HttpResponse::Ok().json(AccountResponse {
            message: "Got user".to_string(),
            user,
        })
    )
}

#[get("/explore")]
pub async fn get_explore_users() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection: &mut diesel::prelude::PgConnection = &mut create_connection();
    let users_result: QueryResult<Vec<User>> = users::table
        .select(User::as_select())
        .load(connection);
    if users_result.is_err() {
        return Ok(
            HttpResponse::Ok().json(Message {
                message: "Failed to get users".to_string(),
            })
        );
    }

    let max_return = 10;
    let all_users = users_result.expect("Failed to get users");

    if all_users.iter().count() <= max_return {
        return Ok(
            HttpResponse::Ok().json(UserExploreMessage {
                message: "Got users".to_string(),
                users: all_users,
            })
        );
    }

    let mut rng = thread_rng();
    let us: Vec<User> = all_users.into_iter().choose_multiple(&mut rng, max_return);

    Ok(
        HttpResponse::Ok().json(UserExploreMessage {
            message: "Got users".to_string(),
            users: us,
        })
    )
}

#[get("/public")]
pub async fn get_profile(
    data: web::Query<AccountID>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let user_id = data.id;
    if user_id <= 0 {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_GATEWAY).json(Message {
                message: "Failed to parse user ID".to_string(),
            })
        );
    }

    let user_option = get_user_from_id(user_id);

    match user_option {
        Some(mut u) => {
            u.token = "".to_string(); // TODO: better solution
            Ok(
                HttpResponse::Ok().json(AccountResponse {
                    message: "Fetched public profile".to_string(),
                    user: u,
                })
            )
        }
        None => {
            Ok(
                HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                    message: "Failed to get user".to_string(),
                })
            )
        }
    }
}

#[get("/posts")]
pub async fn get_users_posts(
    data: web::Query<UserPostsDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let user_option = get_user_from_id(data.user_id);
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let posts = get_user_posts_from_id(user.id);

    Ok(
        HttpResponse::Ok().json(UserPostsMessage {
            message: "Fetched user posts".to_string(),
            posts,
        })
    )
}

#[get("/collections")]
pub async fn get_user_collections(
    data: web::Query<UserPostsDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let user_option = get_user_from_id(data.user_id);
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let _collections = get_user_collections_from_id(user.id);

    Ok(
        HttpResponse::Ok().json(UserCollectionsMessage {
            message: "Fetched user's collections".to_string(),
            collections: _collections,
        })
    )
}

#[get("/comments")]
pub async fn get_users_comments(
    data: web::Query<UserPostsDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let user_option = get_user_from_id(data.user_id);
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let _comments = get_user_comments_from_id(user.id);

    Ok(
        HttpResponse::Ok().json(UserCommentsMessage {
            message: "Fetched user's comments".to_string(),
            comments: _comments,
        })
    )
}

#[get("/search")]
pub async fn get_search_users(
    data: web::Query<SearchRequest>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let mut users_vec: Vec<User> = Vec::new();

    if data.id != 0 {
        let user_result: QueryResult<User> = users::table.find(data.id).first::<User>(connection);
        if user_result.is_ok() {
            let user: User = user_result.unwrap();
            users_vec.push(user);
        }
    }

    let user_results: QueryResult<Vec<User>> = users::table.load(connection);

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

    Ok(
        HttpResponse::Ok().json(SearchRequestMessage {
            message: "Fetched users".to_string(),
            users: users_vec,
        })
    )
}
