use actix_web::{
    get, 
    Responder, 
    web, 
    HttpResponse
};
use diesel::{
    QueryDsl, 
    QueryResult, 
    RunQueryDsl, 
    SelectableHelper,
    
};
use pinned_db::create_connection;
use pinned_db_schema::{
    schema::collections, 
    models::Collection
};
use reqwest::StatusCode;
use crate::{
    collections::dto::{
        GetCollectionDTO,
        GetCollectionMessage
    }, 
    dto::Message
};

#[get("")]
pub async fn get_collection(query: web::Query<GetCollectionDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();

    let collection_result: QueryResult<Collection> = collections::table
        .find(query.collection_id)
        .select(Collection::as_returning())
        .first::<Collection>(connection);

    match collection_result {
        Ok(collection) => {
            let success_message = GetCollectionMessage { message: "Fetched collection".to_string(), collection };
            Ok(HttpResponse::Ok().json(success_message))
        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(error_message))
        }
    }
}
