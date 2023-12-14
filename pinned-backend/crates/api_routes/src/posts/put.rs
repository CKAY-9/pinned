use actix_web::{put, HttpRequest, web, Responder, HttpResponse};
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, QueryResult};
use pinned_db::create_connection;
use pinned_db_schema::{schema::{posts, users}, models::{User, Post}};
use reqwest::StatusCode;

use crate::dto::Message;

use super::dto::UpdatePostDTO;

#[put("")]
pub async fn update_post(request: HttpRequest, data: web::Json<UpdatePostDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let auth_header = request.headers().get("Authorization");
    if auth_header.is_none() {
        let error_message = Message { message: "Failed to parse auth header".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(error_message));
    }

    let auth_header_result = auth_header.unwrap().to_str().unwrap();

    let connection = &mut create_connection();
    let user: QueryResult<User> = users::table.filter(users::token.eq(auth_header_result)).first::<User>(connection);
    match user {
        Ok(user) => {
            let post_result: QueryResult<Post> = posts::table
                .filter(posts::id.eq(data.post_id.clone()))
                .first::<Post>(connection);

            if post_result.is_err() {
                let post_error_message = Message { message: "Failed to get post".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(post_error_message));
            }

            let post_unwrap = post_result.unwrap();
            if post_unwrap.creator != user.id {
                let user_ownership_message = Message { message: "User doesn't own post".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(user_ownership_message));
            }

            let _ = diesel::update(posts::table)
                .filter(posts::id.eq(data.post_id.clone()))
                .set((
                    posts::title.eq(data.title.clone()),
                    posts::description.eq(data.description.clone())
                ))
                .execute(connection);
            
            let success_message = Message { message: "Updated post".to_string() };
            Ok(HttpResponse::Ok().json(success_message))
        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }
}
