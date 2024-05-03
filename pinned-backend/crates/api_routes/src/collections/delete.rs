use actix_web::{ delete, HttpResponse, HttpRequest, Responder, web };
use pinned_db::crud::{collections::{delete_collection_from_id, get_collection_from_id}, users::get_user_from_token};
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use crate::dto::Message;
use super::dto::GetCollectionDTO;

#[delete("")]
pub async fn delete_collection(
    request: HttpRequest,
    data: web::Json<GetCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = extract_header_value(&request, "Authorization");
    if token.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
            message: "Failed to get user token".to_string()
        }));
    }

    let user_option = get_user_from_token(token.unwrap());
    if user_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
            message: "Failed to get user".to_string()
        }));
    }

    let collection_option = get_collection_from_id(data.collection_id);
    if collection_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
            message: "Failed to get collection".to_string()
        }));
    }

    let collection = collection_option.unwrap();
    let user = user_option.unwrap();

    if collection.creator != user.id {
        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(Message {
            message: "Invalid permissions".to_string()
        }));
    }

    let delete = delete_collection_from_id(collection.id);
    match delete {
        true => Ok(HttpResponse::Ok().json(Message {
            message: "Deleted collection".to_string()
        })),
        _ => Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
            message: "Failed to delete collection".to_string()
        }))
    }
    
}
