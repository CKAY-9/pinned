use actix_web::{ put, HttpRequest, Responder, web, HttpResponse };
use pinned_db::crud::{
    collections::{ get_collection_from_id, update_collection_from_id },
    posts::get_post_from_id,
    users::get_user_from_token,
};
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use crate::{ collections::dto::AddToCollectionDTO, dto::Message };
use super::dto::{ AddCollaboratorsDTO, LikeCollectionDTO, UpdateCollectionDTO };

#[put("")]
pub async fn update_collection(
    request: HttpRequest,
    data: web::Json<UpdateCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = extract_header_value(&request, "Authorization");
    if token.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user token".to_string(),
            })
        );
    }

    let user_option = get_user_from_token(token.unwrap());
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let collection_option = get_collection_from_id(data.collection_id);
    if collection_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get collection".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let mut collection = collection_option.unwrap();

    if collection.creator != user.id && !collection.collaborators.contains(&user.id) {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Invalid permissions".to_string(),
            })
        );
    }

    collection.description = data.description.clone();
    collection.name = data.name.clone();
    let collection_update = serde_json
        ::from_str(serde_json::to_string(&collection).unwrap().as_str())
        .unwrap();
    let update_option = update_collection_from_id(collection.id, collection_update);

    if update_option.is_none() {
        let update_message = Message { message: "Failed to update collection".to_string() };
        return Ok(
            HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(update_message)
        );
    }

    Ok(HttpResponse::Ok().json(Message { message: "Updated collection".to_string() }))
}

#[put("/add_collaborator")]
pub async fn add_collaborator_to_collection(
    request: HttpRequest,
    data: web::Json<AddCollaboratorsDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = extract_header_value(&request, "Authorization");
    if token.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user token".to_string(),
            })
        );
    }

    let user_option = get_user_from_token(token.unwrap());
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let collection_option = get_collection_from_id(data.collection_id);
    if collection_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get collection".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let mut collection = collection_option.unwrap();

    if collection.creator != user.id {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Invalid permissions".to_string(),
            })
        );
    }

    // TODO: add user checking
    collection.collaborators.push(data.user_id);

    let collection_update = serde_json
        ::from_str(serde_json::to_string(&collection).unwrap().as_str())
        .unwrap();
    let update_option = update_collection_from_id(collection.id, collection_update);
    match update_option {
        Some(_c) =>
            Ok(
                HttpResponse::Ok().json(Message {
                    message: "Added collaborator".to_string(),
                })
            ),
        None =>
            Ok(
                HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                    message: "Failed to add collaborator".to_string(),
                })
            ),
    }
}

#[put("/add")]
pub async fn update_add_to_collection(
    request: HttpRequest,
    data: web::Json<AddToCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = extract_header_value(&request, "Authorization");
    if token.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user token".to_string(),
            })
        );
    }

    let user_option = get_user_from_token(token.unwrap());
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();

    let collection_option = get_collection_from_id(data.collection_id);
    if collection_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get collection".to_string(),
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

    let mut collection = collection_option.unwrap();
    let post = post_option.unwrap();

    if collection.creator != user.id && !collection.collaborators.contains(&user.id) {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Invalid permissions".to_string(),
            })
        );
    }

    collection.linked_posts.push(post.id);

    let collection_update = serde_json
        ::from_str(serde_json::to_string(&collection).unwrap().as_str())
        .unwrap();
    let update_option = update_collection_from_id(collection.id, collection_update);

    if update_option.is_none() {
        let update_message = Message { message: "Failed to update collection".to_string() };
        return Ok(
            HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(update_message)
        );
    }

    Ok(HttpResponse::Ok().json(Message { message: "Updated collection".to_string() }))
}

#[put("/like")]
pub async fn update_likes_on_collection(
    request: HttpRequest,
    data: web::Json<LikeCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = extract_header_value(&request, "Authorization");
    if token.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user token".to_string(),
            })
        );
    }

    let user_option = get_user_from_token(token.unwrap());
    if user_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();

    let collection_option = get_collection_from_id(data.collection_id);
    if collection_option.is_none() {
        return Ok(
            HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get collection".to_string(),
            })
        );
    }

    let mut collection = collection_option.unwrap();

    let like_type = data.like_type;
    let mut index = 0;
    let mut flag = false;

    match like_type {
        -1 => {
            // Dislike
            for dislike in collection.dislikes.iter() {
                if dislike.to_owned() == user.id {
                    collection.dislikes.remove(index);
                    flag = true;
                    break;
                }
                index += 1;
            }
            if !flag {
                index = 0;
                for like in collection.likes.iter() {
                    if like.to_owned() == user.id {
                        collection.likes.remove(index);
                        break;
                    }
                }
                collection.dislikes.push(user.id);
            }
        }
        1 => {
            // Like
            for like in collection.likes.iter() {
                if like.to_owned() == user.id {
                    collection.likes.remove(index);
                    flag = true;
                    break;
                }
                index += 1;
            }
            if !flag {
                index = 0;
                for dislike in collection.dislikes.iter() {
                    if dislike.to_owned() == user.id {
                        collection.dislikes.remove(index);
                        break;
                    }
                }
                collection.likes.push(user.id);
            }
        }
        _ => {
            // Reset
            for like in collection.likes.iter() {
                if like.to_owned() == user.id {
                    collection.likes.remove(index);
                    break;
                }
                index += 1;
            }
            index = 0;
            for dislike in collection.dislikes.iter() {
                if dislike.to_owned() == user.id {
                    collection.dislikes.remove(index);
                    break;
                }
                index += 1;
            }
        }
    }

    let collection_update = serde_json
        ::from_str(serde_json::to_string(&collection).unwrap().as_str())
        .unwrap();
    let update_option = update_collection_from_id(collection.id, collection_update);
    match update_option {
        Some(_u) =>
            Ok(
                HttpResponse::Ok().json(Message {
                    message: "Liked/Disliked post".to_string(),
                })
            ),
        None =>
            Ok(
                HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                    message: "Failed to like/dislike post".to_string(),
                })
            ),
    }
}
