use actix_web::{ put, web, HttpRequest, HttpResponse, Responder };
use pinned_db::crud::users::{get_user_from_token, update_user_from_id};
use pinned_db_schema::models::NewUser;
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use crate::dto::Message;

use super::dto::UserUpdateDTO;

#[put("")]
pub async fn update_user(
    request: HttpRequest,
    data: web::Json<UserUpdateDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token_option = extract_header_value(&request, "Authorization");
    if token_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
            message: "Failed to get user token".to_string()
        }));
    }

    let user_option = get_user_from_token(token_option.unwrap());
    if user_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
            message: "Failed to get user".to_string()
        }));
    }

    let mut user = user_option.unwrap();
    user.bio = data.bio.clone();
    let user_update = serde_json::from_str::<NewUser>(serde_json::to_string(&user).unwrap().as_str()).unwrap();
    let update = update_user_from_id(user.id, user_update);

    match update {
        Some(u) => {
            Ok(HttpResponse::Ok().json(u))
        },
        None => {
            Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                message: "Failed to update user".to_string()
            }))
        }
    }
}
