use pinned_db_schema::models::{Collection, User};
use serde::{ Deserialize, Serialize };

#[derive(Deserialize)]
pub struct NewCollectionDTO {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct NewCollectionsMessage {
    pub message: String,
    pub collection_id: i32,
}

#[derive(Deserialize)]
pub struct AddCollaboratorsDTO {
    pub user_id: i32,
    pub collection_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateCollectionDTO {
    pub name: String,
    pub description: String,
    pub collection_id: i32,
}

#[derive(Deserialize)]
pub struct GetCollectionDTO {
    pub collection_id: i32,
}

#[derive(Serialize)]
pub struct GetCollaboratorsMessage {
    pub message: String,
    pub collaborators: Vec<User>
}

#[derive(Serialize)]
pub struct GetCollectionMessage {
    pub message: String,
    pub collection: Collection,
}

#[derive(Deserialize)]
pub struct AddToCollectionDTO {
    pub collection_id: i32,
    pub post_id: i32,
}

#[derive(Deserialize)]
pub struct LikeCollectionDTO {
    pub collection_id: i32,
    pub like_type: i32,
}
