use std::time::SystemTime;

use actix_web::{
    post, 
    HttpRequest, 
    HttpResponse, 
    Responder
};
use diesel::{
    ExpressionMethods, 
    QueryDsl, 
    QueryResult, 
    RunQueryDsl
};
use pinned_db::create_connection;
use pinned_db_schema::schema::users::id;
use pinned_db_schema::{
    models::{
        NewUser, 
        User
    },
    schema::{
        self, 
        posts::dsl::*, 
        users::dsl::*
    },
};
use pinned_utils::iso8601;
use reqwest::StatusCode;
use crate::dto::Message;

#[post("/reset")]
pub async fn post_reset_user(
    request: HttpRequest,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let auth_header = request.headers().get("Authorization");
    if auth_header.is_none() {
        let error_message = Message {
            message: "Failed to parse auth header".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .json(error_message));
    }

    let auth_header_result = auth_header.unwrap().to_str().unwrap();

    let connection = &mut create_connection();
    let user: QueryResult<User> = users
        .filter(token.eq(auth_header_result))
        .first::<User>(connection);
    match user {
        Ok(user) => {
            let _ = diesel::update(users)
                .filter(id.eq(user.id))
                .set(&NewUser {
                    username: user.username,
                    bio: "No bio provided".to_string(),
                    joined: iso8601(&SystemTime::now()),
                    avatar: user.avatar,
                    token: user.token,
                    oauth_id: user.oauth_id,
                    collections: Vec::new(),
                })
                .execute(connection);

            let _ = diesel::delete(posts)
                .filter(schema::posts::creator.eq(user.id))
                .execute(connection);

            let _ = diesel::delete(schema::comments::table)
                .filter(schema::comments::creator.eq(user.id))
                .execute(connection);
            
            let _ = diesel::delete(schema::collections::table)
                .filter(schema::collections::creator.eq(user.id))
                .execute(connection);

            Ok(HttpResponse::Ok().body("Reset"))
        }
        Err(e) => {
            println!("{}", e);
            let error_message = Message {
                message: e.to_string(),
            };
            Ok(HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .json(error_message))
        }
    }
}
