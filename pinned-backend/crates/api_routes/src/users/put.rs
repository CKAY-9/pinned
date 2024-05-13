use actix_web::{ put, web, HttpRequest, HttpResponse, Responder };
use pinned_db::crud::{posts::get_post_from_id, users::{get_user_from_token, update_user_from_id}};
use pinned_db_schema::models::NewUser;
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use crate::dto::Message;

use super::dto::{UserPinDTO, UserUpdateDTO};

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

#[put("/pin")]
pub async fn pin_post_to_profile(
    request: HttpRequest,
    data: web::Json<UserPinDTO>
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
    let post_option = get_post_from_id(data.post_id);
    if post_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
            message: "Failed to get post".to_string()
        }));
    }

    let post = post_option.unwrap();
    if post.creator != user.id {
        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
            message: "Invalid permissions".to_string()
        }));
    }

    match data.remove {
        true => {
            for i in 0..user.pinned.len() {
                if user.pinned.get(i).unwrap_or(&0) == &data.post_id {
                    user.pinned.remove(i);
                    break;
                }
            }

            let update = serde_json::from_str::<NewUser>(serde_json::to_string(&user).unwrap().as_str()).unwrap();
            let _ = update_user_from_id(post.id, update);

            Ok(HttpResponse::Ok().json(Message {
                message: "Removed pin from post".to_string()
            }))
        },
        _ => {
            if user.pinned.len() >= 3 {
                return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                    message: "Already at max pin count".to_string()
                }));
            }

            user.pinned.push(post.id);

            let update = serde_json::from_str::<NewUser>(serde_json::to_string(&user).unwrap().as_str()).unwrap();
            let _ = update_user_from_id(post.id, update);

            Ok(HttpResponse::Ok().json(Message {
                message: "Added pin".to_string()
            }))
        }
    }
}