use actix_web::{ post, Responder, HttpRequest, web, HttpResponse };
use pinned_db::crud::{ collections::create_collection, users::get_user_from_token };
use pinned_db_schema::models::NewCollection;
use pinned_utils::extract_header_value;
use reqwest::StatusCode;
use crate::{ collections::dto::{ NewCollectionDTO, GetCollectionMessage }, dto::Message };

#[post("")]
pub async fn create_new_collection(
    request: HttpRequest,
    data: web::Json<NewCollectionDTO>
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
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(Message {
                message: "Failed to get user".to_string(),
            })
        );
    }

    let user = user_option.unwrap();
    let new_collection = NewCollection {
        name: data.name.clone(),
        description: data.description.clone(),
        likes: vec![],
        dislikes: vec![],
        recommended_collections: vec![],
        linked_posts: vec![],
        linked_comments: vec![],
        creator: user.id,
        collaborators: vec![],
    };
    let insert = create_collection(new_collection);
    match insert {
        Some(c) =>
            Ok(
                HttpResponse::Ok().json(GetCollectionMessage {
                    message: "Created collection".to_string(),
                    collection: c,
                })
            ),
        None =>
            Ok(
                HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(Message {
                    message: "Failed to create collection".to_string(),
                })
            ),
    }
}
