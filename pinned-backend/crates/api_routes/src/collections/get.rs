use actix_web::{ get, Responder, web, HttpResponse };
use pinned_db::crud::{collections::get_collection_from_id, users::get_user_from_id};
use pinned_db_schema::models::User;
use reqwest::StatusCode;
use crate::{ collections::dto::{ GetCollectionDTO, GetCollectionMessage, GetCollaboratorsMessage }, dto::Message };

#[get("")]
pub async fn get_collection(
    query: web::Query<GetCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let collection = get_collection_from_id(query.collection_id);
    match collection {
        Some(c) =>
            Ok(
                HttpResponse::Ok().json(GetCollectionMessage {
                    message: "Got collection".to_string(),
                    collection: c,
                })
            ),
        None =>
            Ok(
                HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                    message: "Failed to get collection".to_string(),
                })
            ),
    }
}

#[get("/collaborators")]
pub async fn get_collection_collaborators(
    query: web::Query<GetCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let collection_option = get_collection_from_id(query.collection_id);
    if collection_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
            message: "Failed to get collection".to_string()
        }));
    }

    let collection = collection_option.unwrap();
    let mut users: Vec<User> = vec![];
    for collab_id in collection.collaborators {
        let temp_user = get_user_from_id(collab_id);
        if temp_user.is_none() {
            continue;
        }
        users.push(temp_user.unwrap());
    }

    Ok(HttpResponse::Ok().json(GetCollaboratorsMessage {
        message: "Got collaborators".to_string(),
        collaborators: users
    }))
}