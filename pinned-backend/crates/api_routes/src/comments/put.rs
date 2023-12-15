use actix_web::{
    put, 
    HttpRequest, 
    Responder, 
    web, 
    HttpResponse
};
use diesel::{
    QueryResult,
    RunQueryDsl,
    ExpressionMethods, 
    QueryDsl
};
use pinned_db::create_connection;
use pinned_db_schema::{
    models::{
        User, 
        Comment
    }, 
    schema::{
        users, 
        comments
    }};
use reqwest::StatusCode;
use super::dto::LikeCommentDTO;
use crate::dto::Message;

#[put("/like")]
pub async fn update_likes_on_comment(request: HttpRequest, data: web::Json<LikeCommentDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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
            let comment_result: QueryResult<Comment> = comments::table
                .find(data.comment_id)
                .first::<Comment>(connection);
            
            match comment_result {
                Ok(mut comment) => {
                    let like_type = data.like_type;
                    let mut index = 0; 
                    let mut flag = false;

                    match like_type {
                        -1 => { // Dislike
                            for dislike in comment.dislikes.iter() {
                                if dislike.to_owned() == user.id {
                                    comment.dislikes.remove(index);
                                    flag = true;
                                    break;
                                }
                                index += 1;
                            }
                            if !flag {
                                comment.dislikes.push(user.id);
                            }
                        },
                        1 => { // Like
                            for like in comment.likes.iter() {
                                if like.to_owned() == user.id {
                                    comment.likes.remove(index);
                                    flag = true;
                                    break;
                                }
                                index += 1;
                            }
                            if !flag {
                                comment.likes.push(user.id);
                            }
                        },
                        _ => { // Reset
                            for like in comment.likes.iter() {
                                if like.to_owned() == user.id {
                                    comment.likes.remove(index);
                                    break; 
                                }
                                index += 1;
                            }
                            index = 0;
                            for dislike in comment.dislikes.iter() {
                                if dislike.to_owned() == user.id {
                                    comment.dislikes.remove(index);
                                    break;
                                }
                                index += 1;
                            }
                        }
                    };

                    let update_result = diesel::update(comments::table)
                        .filter(comments::id.eq(comment.id))
                        .set((
                            comments::likes.eq(comment.likes),
                            comments::dislikes.eq(comment.dislikes)
                        ))
                        .execute(connection);

                    match update_result {
                        Ok(update) => {
                            let success_message = Message { message: "Updated comment".to_string() };
                            Ok(HttpResponse::Ok().json(success_message))
                        },
                        Err(e) => {
                            let update_message = Message { message: e.to_string() };
                            Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(update_message))
                        }
                    }
                },
                Err(e) => {
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
