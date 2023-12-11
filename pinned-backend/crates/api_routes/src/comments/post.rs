use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use pinned_db::create_connection;
use pinned_db_schema::{schema::{users::{token, self}, comments}, models::{User, NewComment}};
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
            let new_comment = NewComment {
                content: post.content.clone(),
                post: post.post.clone(),
                creator: user.id,
                likes: vec![],
                dislikes: vec![]
            };

            let insert = diesel::insert_into(comments::table)
                .values(new_comment)
                .execute(connection)
                .expect("Failed to insert comment");

            let success_message = NewCommentMessage { message: "Created new comment".to_string(), comment_id: insert };
            Ok(HttpResponse::Ok().json(success_message))
        },
        Err(e) => {
            println!("{}", e);
            let error_message = Message { message: "Failed to create comment".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(error_message))
        }
    }
}
