use actix_web::{ get, Responder, web, HttpResponse };
use pinned_db::crud::collections::get_collection_from_id;
use reqwest::StatusCode;
use crate::{ collections::dto::{ GetCollectionDTO, GetCollectionMessage }, dto::Message };

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
