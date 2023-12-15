use pinned_db_schema::models::Collection;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Deserialize)]
pub struct NewCollectionDTO {
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct NewCollectionsMessage {
    pub message: String,
    pub collection_id: i32
}

#[derive(Deserialize)]
pub struct GetCollectionDTO {
    pub collection_id: i32
}

#[derive(Serialize)]
pub struct GetCollectionMessage {
    pub message: String,
    pub collection: Collection
}
