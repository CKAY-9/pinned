use actix_web::{get, Responder, HttpResponse, web};
use diesel::{QueryResult, RunQueryDsl, QueryDsl, SelectableHelper};
use pinned_db::create_connection;
use pinned_db_schema::{models::Post, schema::posts};
use reqwest::StatusCode;
use crate::{posts::dto::{GetPostDTO, GetPostMessage}, dto::Message};

#[get("")]
pub async fn get_post(data: web::Json<GetPostDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let post: QueryResult<Post> = posts::table
        .find(data.post_id)
        .select(Post::as_select())
        .first(connection);

    match post {
        Ok(post) => {
            let post_message = GetPostMessage { message: "Fetched post".to_string(), post };
            Ok(HttpResponse::Ok().json(post_message))
        },
        Err(e) => {
            println!("{}", e);
            let error_message = Message { message: "Failed to get post".to_string() };
            Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(error_message))
        }
    }
}
