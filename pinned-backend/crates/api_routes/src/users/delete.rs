use actix_web::{delete, Responder, HttpRequest, HttpResponse};
use diesel::{ExpressionMethods, QueryResult, QueryDsl};
use pinned_db::create_connection;
use pinned_db_schema::{schema::users::{token, self}, models::User};
use reqwest::StatusCode;
use crate::dto::Message;

#[delete("/")]
pub async fn delete_user(request: HttpRequest) -> impl Responder {
    let auth_header = request.headers().get("Authorization");
    if auth_header.is_none() {
        let error_message = Message { message: "Failed to parse auth header".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(error_message));
    }

    let auth_header_result = auth_header.unwrap().to_str().unwrap();

    let connection = &mut create_connection();
    let user: QueryResult<User> = users::table.filter(token.eq(auth_header_result.to_string())).first::<User>(connection);
    match user {
        Ok(user) => { 
            let _ = diesel::delete(users::table.filter(id.eq(user.id))).execute(connection)?;
            let success_message = Message { message: "Deleted user account".to_string() };
            Ok(HttpResponse::Ok().json(success_message))
        },
        Err(e) => {
            let error_message = Message { message: "Failed to find user account".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(error_message))
        }
    }
}
