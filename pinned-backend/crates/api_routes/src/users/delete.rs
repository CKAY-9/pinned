use crate::dto::Message;
use actix_web::{ delete, HttpRequest, HttpResponse, Responder };
use pinned_db::crud::users::{delete_user_from_id, get_user_from_token};
use pinned_utils::extract_header_value;
use reqwest::StatusCode;

#[delete("")]
pub async fn delete_user(
    request: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token_option = extract_header_value(&request, "Authorization");
    if token_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
            message: "Failed to get user token".to_string()
        }));
    }

    let user_option = get_user_from_token(token_option.unwrap());
    if user_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
            message: "Failed to get user".to_string()
        }));
    }

    let user = user_option.unwrap();
    let _ = delete_user_from_id(user.id);

    Ok(HttpResponse::Ok().json(Message {
        message: "Deleted user".to_string()
    }))
}
