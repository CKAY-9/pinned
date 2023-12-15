use std::time::SystemTime;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods, SelectableHelper};
use pinned_db::create_connection;
use pinned_db_schema::{schema::{users::{token, self}, comments::{id, self}, posts::{self}}, models::{User, NewComment, Post}};
use pinned_utils::iso8601;
use reqwest::StatusCode;

use crate::dto::Message;

use super::dto::{NewCommentDTO, NewCommentMessage};

#[post("")]
pub async fn create_new_comment(request: HttpRequest, post: web::Json<NewCommentDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let parent_post: QueryResult<Post> = posts::table
                .filter(posts::id.eq(post.post.clone()))
                .select(Post::as_select())
                .first::<Post>(connection);

            if parent_post.is_err() {
                let post_error_message = Message { message: "Failed to get post".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(post_error_message));
            }

            let mut post_unwrap = parent_post.unwrap();

            let new_comment = NewComment {
                content: post.content.clone(),
                posted: iso8601(&SystemTime::now()),
                post: post.post.clone(),
                creator: user.id,
                likes: vec![],
                dislikes: vec![]
            };

            let insert_result = diesel::insert_into(comments::table)
                .values(new_comment)
                .get_result::<(i32, i32, i32, String, String, Vec<i32>, Vec<i32>)>(connection);

            if insert_result.is_err() {
                let insert_error_message = Message { message: "Failed to insert post".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(insert_error_message));
            }

            let insert = insert_result.unwrap();

            post_unwrap.comments.push(0);

            let _ = diesel::update(posts::table)
                .filter(posts::id.eq(post.post.clone()))
                .set(posts::comments.eq(post_unwrap.comments))
                .execute(connection);

            let success_message = NewCommentMessage { message: "Created new comment".to_string(), comment_id: insert.0 };
            Ok(HttpResponse::Ok().json(success_message))
        },
        Err(e) => {
            println!("{}", e);
            let error_message = Message { message: "Failed to create comment".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(error_message))
        }
    }
}
