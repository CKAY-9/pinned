use std::time::SystemTime;
use crate::{ dto::Message, posts::dto::{ NewPostDTO, GetPostMessage, GetPostDTO } };
use actix_web::{ post, web, HttpRequest, HttpResponse, Responder };
use pinned_db::crud::{ posts::create_post, users::{get_user_from_token, update_user_from_id} };
use pinned_db_schema::models::{NewPost, NewUser};
use pinned_utils::{ extract_header_value, iso8601 };
use reqwest::StatusCode;

#[post("")]
pub async fn create_new_post(
    request: HttpRequest,
    post: web::Json<NewPostDTO>
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

    let user = user_option.unwrap();

    let new_post = NewPost {
        title: post.title.clone(),
        description: post.description.clone(),
        file_id: post.file_id.clone(),
        posted: iso8601(&SystemTime::now()),
        creator: user.id,
        dislikes: vec![],
        likes: vec![],
        comments: vec![],
    };

    let post = create_post(new_post);
    if post.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                message: "Failed to create post".to_string(),
            })
        );
    }

    Ok(
        HttpResponse::Ok().json(GetPostMessage {
            message: "Created post".to_string(),
            post: post.unwrap(),
        })
    )
}

#[post("/favourite")]
pub async fn favourite_post(
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

    let mut user = user_option.unwrap();
    match user.favourites.contains(&data.post_id) {
        true => {
            for i in 0..user.favourites.len() {
                if user.favourites.get(i).unwrap_or(&0) == &data.post_id {
                    user.favourites.remove(i);
                    break;
                }
            }

            let update = serde_json::from_str::<NewUser>(serde_json::to_string(&user).unwrap().as_str()).unwrap();
            let _ = update_user_from_id(user.id, update);
            Ok(HttpResponse::Ok().json(Message {
                message: "Unfavourited post".to_string()
            }))
        },
        _ => {
            user.favourites.push(data.post_id);

            let update = serde_json::from_str::<NewUser>(serde_json::to_string(&user).unwrap().as_str()).unwrap();
            let _ = update_user_from_id(user.id, update);
            Ok(HttpResponse::Ok().json(Message {
                message: "Favourited post".to_string()
            }))
        }
    }
}