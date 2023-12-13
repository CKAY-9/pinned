use actix_web::{delete, HttpRequest, web, Responder, HttpResponse};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, QueryResult, SelectableHelper};
use pinned_db::create_connection;
use pinned_db_schema::{schema::{users::{self, token}, comments::{self}}, models::{User, Comment}};
use reqwest::StatusCode;

use crate::dto::Message;

use super::dto::GetCommentDTO;

#[delete("")]
pub async fn delete_comment(request: HttpRequest, data: web::Json<GetCommentDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let comment_result: QueryResult<Comment> = comments::table
                .find(data.comment_id)
                .select(Comment::as_select())
                .first(connection);

            if comment_result.is_err() {
                let comment_failed_message = Message { message: "Failed to get comment".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(comment_failed_message));
            }

            let comment = comment_result.unwrap();
            if comment.creator != user.id {
                let user_doesnt_own_comment_message = Message { message: "User doesn't own comment".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(user_doesnt_own_comment_message));
            }

            let _ = diesel::delete(comments::table.find(data.comment_id)).execute(connection)?;
            let message = Message { message: "Deleted comment".to_string() };
            Ok(HttpResponse::Ok().json(message))
        },
        Err(e) => {
            println!("{}", e);
            let error_message = Message { message: "Failed to get user".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }
}
