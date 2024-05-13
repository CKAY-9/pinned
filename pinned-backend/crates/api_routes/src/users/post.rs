use std::time::SystemTime;

use crate::dto::Message;
use actix_web::{ post, HttpRequest, HttpResponse, Responder };
use diesel::{ ExpressionMethods, RunQueryDsl };
use pinned_db::create_connection;
use pinned_db::crud::users::{get_user_from_token, update_user_from_id};
use pinned_db_schema::{ models::NewUser, schema::{ self, posts::dsl::* } };
use pinned_utils::{extract_header_value, iso8601};
use reqwest::StatusCode;

#[post("/reset")]
pub async fn post_reset_user(
    request: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token_option = extract_header_value(&request, "Authorization");
    if token_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user token".to_string(),
            })
        );
    }

    let user_option = get_user_from_token(token_option.unwrap());
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let reset_user = NewUser {
        username: user.username,
        bio: "No bio provided".to_string(),
        joined: iso8601(&SystemTime::now()),
        avatar: user.avatar,
        token: user.token,
        oauth_id: user.oauth_id,
        collections: Vec::new(),
        favourites: vec![],
        pinned: vec![]
    };

    let _ = update_user_from_id(user.id, reset_user);

    // delete all user contents
    let connection = &mut create_connection();
    let _ = diesel::delete(posts).filter(schema::posts::creator.eq(user.id)).execute(connection);
    let _ = diesel
        ::delete(schema::comments::table)
        .filter(schema::comments::creator.eq(user.id))
        .execute(connection);
    let _ = diesel
        ::delete(schema::collections::table)
        .filter(schema::collections::creator.eq(user.id))
        .execute(connection);

    Ok(HttpResponse::Ok().json(Message {
        message: "Reset".to_string()
    }))
}
