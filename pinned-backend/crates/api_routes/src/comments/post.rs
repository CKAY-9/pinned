use actix_web::{ post, web, HttpRequest, HttpResponse, Responder };
use pinned_db::crud::{
    comments::create_comment,
    posts::{ get_post_from_id, update_post_from_id },
    users::get_user_from_token,
};
use pinned_db_schema::models::NewComment;
use pinned_utils::{ extract_header_value, iso8601 };
use reqwest::StatusCode;
use std::time::SystemTime;
use crate::dto::Message;
use super::dto::{ NewCommentDTO, NewCommentMessage };

#[post("")]
pub async fn create_new_comment(
    request: HttpRequest,
    data: web::Json<NewCommentDTO>
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
                message: "Failed to get parent post".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let mut post = post_option.unwrap();

    let new_comment = NewComment {
        content: data.content.clone(),
        posted: iso8601(&SystemTime::now()),
        post: data.post_id.clone(),
        creator: user.id,
        likes: vec![],
        dislikes: vec![],
    };
    let insert_option = create_comment(new_comment);
    if insert_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                message: "Failed to create comment".to_string(),
            })
        );
    }

    let insert = insert_option.unwrap();

    post.comments.push(insert.id);
    let post_update = serde_json::from_str(serde_json::to_string(&post).unwrap().as_str()).unwrap();
    let _ = update_post_from_id(post.id, post_update);

    Ok(
        HttpResponse::Ok().json(NewCommentMessage {
            message: "Created new comment".to_string(),
            comment_id: insert.id,
        })
    )
}
