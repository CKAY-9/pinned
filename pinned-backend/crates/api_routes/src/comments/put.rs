use actix_web::{ put, HttpRequest, Responder, web, HttpResponse };
use pinned_db::crud::{
    comments::{ get_comment_from_id, update_comment_from_id },
    users::get_user_from_token,
};
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use super::dto::LikeCommentDTO;
use crate::dto::Message;

#[put("/like")]
pub async fn update_likes_on_comment(
    request: HttpRequest,
    data: web::Json<LikeCommentDTO>
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
                message: "Failed to get parent post".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let mut comment = comment_option.unwrap();

    let like_type = data.like_type;
    let mut index = 0;
    let mut flag = false;

    match like_type {
        -1 => {
            // Dislike
            for dislike in comment.dislikes.iter() {
                if dislike.to_owned() == user.id {
                    comment.dislikes.remove(index);
                    flag = true;
                    break;
                }
                index += 1;
            }
            if !flag {
                comment.dislikes.push(user.id);
            }
        }
        1 => {
            // Like
            for like in comment.likes.iter() {
                if like.to_owned() == user.id {
                    comment.likes.remove(index);
                    flag = true;
                    break;
                }
                index += 1;
            }
            if !flag {
                comment.likes.push(user.id);
            }
        }
        _ => {
            // Reset
            for like in comment.likes.iter() {
                if like.to_owned() == user.id {
                    comment.likes.remove(index);
                    break;
                }
                index += 1;
            }
            index = 0;
            for dislike in comment.dislikes.iter() {
                if dislike.to_owned() == user.id {
                    comment.dislikes.remove(index);
                    break;
                }
                index += 1;
            }
        }
    }

    let comment_update = serde_json
        ::from_str(serde_json::to_string(&comment).unwrap().as_str())
        .unwrap();
    let _ = update_comment_from_id(comment.id, comment_update);

    Ok(
        HttpResponse::Ok().json(Message {
            message: "Liked/disliked comment".to_string(),
        })
    )
}
