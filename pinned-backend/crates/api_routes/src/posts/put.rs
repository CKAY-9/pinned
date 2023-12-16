use actix_web::{
    put, 
    web, 
    HttpRequest, 
    HttpResponse, 
    Responder
};
use diesel::{
    ExpressionMethods, 
    QueryDsl, 
    QueryResult, 
    RunQueryDsl
};
use pinned_db::create_connection;
use pinned_db_schema::{
    models::{
        Post, 
        User
    },
    schema::{
        posts, 
        users
    },
};
use reqwest::StatusCode;
use crate::dto::Message;
use super::dto::{
    UpdatePostDTO, 
    LikePostDTO
};

#[put("")]
pub async fn update_post(
    request: HttpRequest,
    data: web::Json<UpdatePostDTO>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let post_result: QueryResult<Post> = posts::table
                .filter(posts::id.eq(data.post_id.clone()))
                .first::<Post>(connection);

            if post_result.is_err() {
                let post_error_message = Message {
                    message: "Failed to get post".to_string(),
                };
                return Ok(HttpResponse::Ok()
                    .status(StatusCode::NOT_FOUND)
                    .json(post_error_message));
            }

            let post_unwrap = post_result.unwrap();
            if post_unwrap.creator != user.id {
                let user_ownership_message = Message {
                    message: "User doesn't own post".to_string(),
                };
                return Ok(HttpResponse::Ok()
                    .status(StatusCode::UNAUTHORIZED)
                    .json(user_ownership_message));
            }

            let _ = diesel::update(posts::table)
                .filter(posts::id.eq(data.post_id.clone()))
                .set((
                    posts::title.eq(data.title.clone()),
                    posts::description.eq(data.description.clone()),
                ))
                .execute(connection);

            let success_message = Message {
                message: "Updated post".to_string(),
            };
            Ok(HttpResponse::Ok().json(success_message))
        }
        Err(e) => {
            let error_message = Message {
                message: e.to_string(),
            };
            Ok(HttpResponse::Ok()
                .status(StatusCode::UNAUTHORIZED)
                .json(error_message))
        }
    }
}

#[put("/like")]
pub async fn update_likes_on_post(request: HttpRequest, data: web::Json<LikePostDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let post: QueryResult<Post> = posts::table
                .find(data.post_id)
                .first::<Post>(connection);
            
            match post {
                Ok(mut post) => {
                    let like_type = data.like_type;
                    let mut index = 0; 
                    let mut flag = false;

                    match like_type {
                        -1 => { // Dislike
                            for dislike in post.dislikes.iter() {
                                if dislike.to_owned() == user.id {
                                    post.dislikes.remove(index);
                                    flag = true;
                                    break;
                                }
                                index += 1;
                            }
                            if !flag {
                                index = 0;
                                for like in post.likes.iter() {
                                    if like.to_owned() == user.id {
                                        post.likes.remove(index);
                                        break;
                                    }
                                }
                                post.dislikes.push(user.id);
                            }
                        },
                        1 => { // Like
                            for like in post.likes.iter() {
                                if like.to_owned() == user.id {
                                    post.likes.remove(index);
                                    flag = true;
                                    break;
                                }
                                index += 1;
                            }
                            if !flag {
                                index = 0;
                                for dislike in post.dislikes.iter() {
                                    if dislike.to_owned() == user.id {
                                        post.dislikes.remove(index);
                                        break;
                                    }
                                }
                                post.likes.push(user.id);
                            }
                        },
                        _ => { // Reset
                            for like in post.likes.iter() {
                                if like.to_owned() == user.id {
                                    post.likes.remove(index);
                                    break; 
                                }
                                index += 1;
                            }
                            index = 0;
                            for dislike in post.dislikes.iter() {
                                if dislike.to_owned() == user.id {
                                    post.dislikes.remove(index);
                                    break;
                                }
                                index += 1;
                            }
                        }
                    };

                    let update_result = diesel::update(posts::table)
                        .filter(posts::id.eq(post.id))
                        .set((
                            posts::likes.eq(post.likes),
                            posts::dislikes.eq(post.dislikes)
                        ))
                        .execute(connection);

                    match update_result {
                        Ok(_update) => {
                            let success_message = Message { message: "Updated post".to_string() };
                            Ok(HttpResponse::Ok().json(success_message))
                        },
                        Err(e) => {
                            let update_message = Message { message: e.to_string() };
                            Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(update_message))
                        }
                    }
                },
                Err(_e) => {
                    let post_message = Message { message: "Failed to get post".to_string() };
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
