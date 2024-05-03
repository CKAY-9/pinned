use crate::dto::Message;
use actix_web::{ delete, web, HttpRequest, HttpResponse, Responder };
use pinned_db::crud::{
    posts::{ delete_post_from_id, get_post_from_id },
    users::get_user_from_token,
};
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use super::dto::GetPostDTO;

#[delete("")]
pub async fn delete_post(
    request: HttpRequest,
    data: web::Json<GetPostDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token_option = extract_header_value(&request, "Authorization");
    if token_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user token".to_string(),
            })
        );
    }

    let user_option = get_user_from_token(token_option.unwrap());
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let post_option = get_post_from_id(data.post_id);
    if post_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get post".to_string(),
            })
        );
    }

    let post = post_option.unwrap();
    let user = user_option.unwrap();

    if post.creator != user.id {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Invalid permissions".to_string(),
            })
        );
    }

    let delete = delete_post_from_id(post.id);
    match delete {
        true =>
            Ok(
                HttpResponse::Ok().json(Message {
                    message: "Deleted post".to_string(),
                })
            ),
        _ =>
            Ok(
                HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                    message: "Failed to delete post".to_string(),
                })
            ),
    }
}
