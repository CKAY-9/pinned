use actix_web::{
    put, 
    HttpRequest, 
    Responder, 
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
        Collection, 
        Post
    }, 
    schema::{
        users, 
        posts, 
        collections
    }
};
use reqwest::StatusCode;
use crate::{
    collections::dto::AddToCollectionDTO, 
    dto::Message
};
use super::dto::{
    UpdateCollectionDTO, 
    LikeCollectionDTO
};

#[put("")]
pub async fn update_collection(request: HttpRequest, data: web::Json<UpdateCollectionDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
    let user_result: QueryResult<User> = users::table
        .filter(users::token.eq(auth_header_result))
        .first::<User>(connection);
    match user_result {
        Ok(user) => {
            let collection_result: QueryResult<Collection> = collections::table
                .find(data.collection_id)
                .first::<Collection>(connection);

            match collection_result {
                Ok(collection) => {
                    if collection.creator != user.id {
                        let ownership_message = Message { message: "User doesn't own collection".to_string() };
                        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(ownership_message));
                    }

                    let update_result = diesel::update(collections::table)
                        .filter(collections::id.eq(data.collection_id))
                        .set((
                            collections::name.eq(data.name.clone()),
                            collections::description.eq(data.description.clone())
                        ))
                        .execute(connection);

                    match update_result {
                        Ok(_update) => {
                            let success_message = Message { message: "Updated collection".to_string() };
                            Ok(HttpResponse::Ok().json(success_message))
                        }, 
                        Err(_e) => {
                            let update_message = Message { message: "Failed to update collection".to_string() };
                            Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(update_message))
                        }
                    }
                },
                Err(_e) => {
                    let collection_message = Message { message: "Failed to get collection".to_string() };
                    Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(collection_message))
                }
            }

            
        },
        Err(_e) => {
            let error_message = Message { message: "Failed to get user".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }
}

#[put("/add")]
pub async fn update_add_to_collection(request: HttpRequest, data: web::Json<AddToCollectionDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
    let user_result: QueryResult<User> = users::table
        .filter(users::token.eq(auth_header_result))
        .first::<User>(connection);

    if user_result.is_err() {
        let user_message = Message { message: "Failed to get user".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(user_message));
    }

    let user = user_result.unwrap();

    let collection_result: QueryResult<Collection> = collections::table
        .find(data.collection_id)
        .first::<Collection>(connection);

    if collection_result.is_err() {
        let collection_message = Message { message: "Failed to get collection".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(collection_message));
    }
    
    let post_result: QueryResult<Post> = posts::table
        .find(data.post_id)
        .first::<Post>(connection);

    if post_result.is_err() {
        let post_message = Message { message: "Failed to get post".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(post_message));
    }
    
    let mut collection = collection_result.unwrap();
    if collection.creator != user.id {
        let owner_message = Message { message: "User doesn't own collection".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(owner_message));
    }

    let post = post_result.unwrap();
    collection.linked_posts.push(post.id);

    let update_result = diesel::update(collections::table)
        .filter(collections::id.eq(data.collection_id))
        .set(
            collections::linked_posts.eq(collection.linked_posts)
        )
        .execute(connection);

    if update_result.is_err() {
        let update_message = Message { message: "Failed to update collection".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(update_message));
    }

    let success_message = Message { message: "Updated collection".to_string() };
    Ok(HttpResponse::Ok().json(success_message))
}

#[put("/like")]
pub async fn update_likes_on_collection(request: HttpRequest, data: web::Json<LikeCollectionDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let collection: QueryResult<Collection> = collections::table
                .find(data.collection_id)
                .first::<Collection>(connection);
            
            match collection {
                Ok(mut collection ) => {
                    let like_type = data.like_type;
                    let mut index = 0; 
                    let mut flag = false;

                    match like_type {
                        -1 => { // Dislike
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
                        },
                        1 => { // Like
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
                        },
                        _ => { // Reset
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
                    };

                    let update_result = diesel::update(collections::table)
                        .filter(collections::id.eq(collection.id))
                        .set((
                            collections::likes.eq(collection.likes),
                            collections::dislikes.eq(collection.dislikes)
                        ))
                        .execute(connection);

                    match update_result {
                        Ok(_update) => {
                            let success_message = Message { message: "Updated collection".to_string() };
                            Ok(HttpResponse::Ok().json(success_message))
                        },
                        Err(e) => {
                            let update_message = Message { message: e.to_string() };
                            Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(update_message))
                        }
                    }
                },
                Err(_e) => {
                    let post_message = Message { message: "Failed to get collection".to_string() };
                    Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(post_message))
                }
            }
        },
        Err(e) => {
            let error_message = Message { message: e.to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).json(error_message))
        }
    }
}
