use actix_web::{get, Responder, HttpResponse, web};
use chrono::{Local, DateTime};
use diesel::{QueryResult, RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use pinned_db::create_connection;
use pinned_db_schema::{models::Post, schema::posts};
use reqwest::StatusCode;
use crate::{posts::dto::{GetPostDTO, GetPostMessage}, dto::Message};

#[get("")]
pub async fn get_post(data: web::Query<GetPostDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
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

#[get("/pinned")]
pub async fn get_today_pinned() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let posts: QueryResult<Vec<Post>> = posts::table.order(posts::likes.desc()).load(connection);
    if posts.is_err() {
        let posts_error_message = Message { message: "Failed to get posts".to_string() };
        return Ok(HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(posts_error_message));
    }

    let posts_unwrap = posts.unwrap();

    let dt = Local::now();
    let naive_utc = dt.naive_utc();
    let offset = dt.offset().clone();
    let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);

    for post in posts_unwrap {
        let date_posted: iso8601::DateTime = iso8601::datetime(post.posted.as_str()).unwrap();    
        // TODO: purge posts older than a day
    }

    Ok(HttpResponse::Ok().body("test"))
}
