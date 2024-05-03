use actix_web::{ delete, web, HttpRequest, HttpResponse, Responder };
use pinned_db::crud::{
    comments::{ delete_comment_from_id, get_comment_from_id },
    users::get_user_from_token,
};
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use crate::dto::Message;
use super::dto::GetCommentDTO;

#[delete("")]
pub async fn delete_comment(
    request: HttpRequest,
    data: web::Json<GetCommentDTO>
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

    let comment_option = get_comment_from_id(data.comment_id);
    if comment_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get comment".to_string(),
            })
        );
    }

    let comment = comment_option.unwrap();
    let user = user_option.unwrap();

    if comment.creator != user.id {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Invalid permissions".to_string(),
            })
        );
    }

    let delete = delete_comment_from_id(comment.id);
    match delete {
        true =>
            Ok(
                HttpResponse::Ok().json(Message {
                    message: "Deleted comment".to_string(),
                })
            ),
        _ =>
            Ok(
                HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                    message: "Failed to delete comment".to_string(),
                })
            ),
    }
}
