use actix_web::{get, Responder, web, HttpResponse};
use diesel::{QueryResult, RunQueryDsl, QueryDsl, SelectableHelper};
use pinned_db::create_connection;
use pinned_db_schema::{models::Comment, schema::comments};
use reqwest::StatusCode;
use crate::{comments::dto::{GetCommentDTO, GetCommentMessage}, dto::Message};

#[get("")]
pub async fn get_comment(data: web::Query<GetCommentDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let comment: QueryResult<Comment> = comments::table
        .find(data.comment_id)
        .select(Comment::as_select())
        .first(connection);

    match comment {
        Ok(comment) => {
            let comment_response = GetCommentMessage { message: "Fetched comment".to_string(), comment };
            Ok(HttpResponse::Ok().json(comment_response))
        },
        Err(e) => {
            println!("{}", e);
            let error_message = Message { message: "Failed to get comment".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(error_message))
        }
    }
}
