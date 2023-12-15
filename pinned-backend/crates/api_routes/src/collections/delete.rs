use actix_web::{
    delete, 
    HttpResponse, 
    HttpRequest, 
    Responder, 
    web
};
use diesel::{
    RunQueryDsl, 
    QueryResult, 
    QueryDsl,
    ExpressionMethods
};
use pinned_db::create_connection;
use pinned_db_schema::{
    models::{
        User, 
        Collection
    }, 
    schema::{
        users, 
        collections
    }
};
use reqwest::StatusCode;
use crate::dto::Message;
use super::dto::GetCollectionDTO;

#[delete("")]
pub async fn delete_collection(request: HttpRequest, data: web::Json<GetCollectionDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let auth_header = request.headers().get("Authorization");
    if auth_header.is_none() {
        let error_message = Message {
            message: "Failed to parse auth header".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .json(error_message));
    }

    let auth_header_result = auth_header.unwrap().to_str().unwrap();

    let connection = &mut create_connection();
    let user: QueryResult<User> = users::table
        .filter(users::token.eq(auth_header_result))
        .first::<User>(connection);
    match user {
        Ok(user) => {
            let collection_result: QueryResult<Collection> = collections::table
                .find(data.collection_id)
                .first::<Collection>(connection);
            match collection_result {
                Ok(collection) => {
                    if collection.creator != user.id {
                        let ownership_message = Message { message: "Use doesn't own collection".to_string() };
                        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(ownership_message));
                    } 

                    let delete_result = diesel::delete(collections::table)
                        .filter(collections::id.eq(data.collection_id))
                        .execute(connection);

                    match delete_result {
                        Ok(delete) => {
                            let success_message = Message { message: "Deleted collection".to_string() };
                            Ok(HttpResponse::Ok().json(success_message))
                        },
                        Err(e) => {
                            let delete_message = Message { message: e.to_string() };
                            Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(delete_message))
                        }
                    }
                },
                Err(e) => {
                    let collection_message = Message { message: "Failed to get collection".to_string() };
                    Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(collection_message))
                }
            }
        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }
}
