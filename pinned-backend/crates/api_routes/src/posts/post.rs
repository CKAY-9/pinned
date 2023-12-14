use std::time::SystemTime;

use actix_web::{post, Responder, HttpResponse, web, HttpRequest};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, QueryResult, SelectableHelper};
use pinned_db::create_connection;
use pinned_db_schema::{models::{User, NewPost, Post}, schema::{users::{self, token}, posts}};
use pinned_utils::iso8601;
use reqwest::StatusCode;
use crate::{posts::dto::{NewPostDTO, NewPostOTD}, dto::Message};

#[post("")]
pub async fn create_new_post(request: HttpRequest, post: web::Json<NewPostDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let auth_header = request.headers().get("Authorization");
    if auth_header.is_none() {
        let error_message = Message { message: "Failed to parse auth header".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(error_message));
    }

    let auth_header_result = auth_header.unwrap().to_str().unwrap();

    let connection = &mut create_connection();
    let user: QueryResult<User> = users::table.filter(token.eq(auth_header_result)).first::<User>(connection);
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
                comments: vec![]
            };

            let insert = diesel::insert_into(posts::table)
                .values(new_post)
                .returning(Post::as_returning())
                .execute(connection)
                .expect("Failed to insert user");

            let success_message = NewPostOTD { message: "Created new post!".to_string(), post_id: insert };
            Ok(HttpResponse::Ok().json(success_message))

        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }        
}
