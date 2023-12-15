use actix_web::{
    post, 
    Responder, 
    HttpRequest, 
    web, 
    HttpResponse
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
        NewCollections
    }, 
    schema::{
        users, 
        collections
    }
};
use reqwest::StatusCode;
use crate::{
    collections::dto::{
        NewCollectionDTO,
        NewCollectionsMessage
    }, 
    dto::Message
};

#[post("")]
pub async fn create_new_collection(request: HttpRequest, data: web::Json<NewCollectionDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let new_collection = NewCollections {
                name: data.name.clone(),
                description: data.description.clone(),
                likes: vec![],
                dislikes: vec![],
                recommended_collections: vec![],
                linked_posts: vec![],
                linked_comments: vec![],
                creator: user.id
            };

            let insert_result = diesel::insert_into(collections::table)
                .values(new_collection)
                .get_result::<(i32, String, String, Vec<i32>, Vec<i32>, Vec<i32>, i32, Vec<i32>, Vec<i32>)>(connection);

            if insert_result.is_err() {
                let insert_message = Message { message: "Failed to create collection".to_string() };
                return Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(insert_message));
            }

            let insert_unwrap = insert_result.unwrap();

            let success_message = NewCollectionsMessage { message: "Created collection".to_string(), collection_id: insert_unwrap.0 };
            Ok(HttpResponse::Ok().json(success_message))
        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }
}
