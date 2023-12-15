use crate::dto::Message;
use actix_web::{delete, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use pinned_db::create_connection;
use pinned_db_schema::schema::users::dsl::*;
use pinned_db_schema::{models::User, schema::users};
use reqwest::StatusCode;

#[delete("")]
pub async fn delete_user(
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
            let _ = diesel::delete(users::table.find(user.id)).execute(connection)?;
            let success_message = Message {
                message: "Deleted user account".to_string(),
            };
            Ok(HttpResponse::Ok().json(success_message))
        }
        Err(e) => {
            let error_message = Message {
                message: e.to_string(),
            };
            Ok(HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .json(error_message))
        }
    }
}
