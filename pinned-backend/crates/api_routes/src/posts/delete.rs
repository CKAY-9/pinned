use actix_web::{delete, HttpRequest, web, Responder, HttpResponse};
use diesel::{QueryResult, RunQueryDsl, SelectableHelper, QueryDsl, ExpressionMethods};
use pinned_db::create_connection;
use pinned_db_schema::{models::{User, Post}, schema::{users::{self, token}, posts}};
use reqwest::StatusCode;
use crate::dto::Message;

use super::dto::GetPostDTO;

#[delete("")]
pub async fn delete_post(request: HttpRequest, data: web::Json<GetPostDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let post_result: QueryResult<Post> = posts::table
                .find(data.post_id)
                .select(Post::as_select())
                .first(connection);

            if post_result.is_err() {
                let post_failed_message = Message { message: "Failed to get post".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(post_failed_message));
            }

            let post = post_result.unwrap();
            if post.creator != user.id {
                let user_doesnt_own_post_message = Message { message: "User doesn't own post".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(user_doesnt_own_post_message));
            }

            let _ = diesel::delete(posts::table.find(data.post_id)).execute(connection)?;
            let message = Message { message: "Deleted post".to_string() };
            Ok(HttpResponse::Ok().json(message))
        },
        Err(e) => {
            println!("{}", e);
            let error_message = Message { message: "Failed to get user".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }
}
