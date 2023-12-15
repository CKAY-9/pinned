use std::time::SystemTime;
use crate::{
    dto::Message,
    posts::dto::{
        NewPostDTO, 
        NewPostOTD
    },
};
use actix_web::{
    post, 
    web, 
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
use pinned_db_schema::{
    models::{
        NewPost, 
        User
    },
    schema::{
        posts,
        users::{
            self, 
            token
        },
    },
};
use pinned_utils::iso8601;
use reqwest::StatusCode;

#[post("")]
pub async fn create_new_post(
    request: HttpRequest,
    post: web::Json<NewPostDTO>,
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
    let user: QueryResult<User> = users::table
        .filter(token.eq(auth_header_result))
        .first::<User>(connection);
    match user {
        Ok(user) => {
            let new_post = NewPost {
                title: post.title.clone(),
                description: post.description.clone(),
                file_id: post.file_id.clone(),
                posted: iso8601(&SystemTime::now()),
                creator: user.id,
                dislikes: vec![],
                likes: vec![],
                comments: vec![],
            };

            let insert_result = diesel::insert_into(posts::table)
                .values(new_post)
                .get_result::<(
                    i32,
                    String,
                    String,
                    String,
                    String,
                    i32,
                    Vec<i32>,
                    Vec<i32>,
                    Vec<i32>,
                )>(connection);

            if insert_result.is_err() {
                let insert_message = Message {
                    message: "Failed to create user".to_string(),
                };
                return Ok(HttpResponse::Ok()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(insert_message));
            }

            let insert = insert_result.unwrap();

            let success_message = NewPostOTD {
                message: "Created new post!".to_string(),
                post_id: insert.0,
            };
            Ok(HttpResponse::Ok().json(success_message))
        }
        Err(e) => {
            let error_message = Message {
                message: e.to_string(),
            };
            Ok(HttpResponse::Ok()
                .status(StatusCode::UNAUTHORIZED)
                .json(error_message))
        }
    }
}
