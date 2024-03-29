use crate::{
    dto::Message,
    posts::dto::{
        GetPostDTO, 
        GetPostMessage,
        SearchPostsDTO,
        SearchPostsMessages
    },
};
use actix_web::{
    get, 
    web, 
    HttpResponse, 
    Responder
};
use chrono::{
    DateTime, 
    Local
};
use diesel::{
    ExpressionMethods, 
    QueryDsl, 
    QueryResult, 
    RunQueryDsl, 
    SelectableHelper
};
use pinned_db::create_connection;
use pinned_db_schema::{
    models::Post, 
    schema::posts
};
use reqwest::StatusCode;

#[get("")]
pub async fn get_post(
    data: web::Query<GetPostDTO>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let post: QueryResult<Post> = posts::table
        .find(data.post_id)
        .select(Post::as_select())
        .first(connection);

    match post {
        Ok(post) => {
            let post_message = GetPostMessage {
                message: "Fetched post".to_string(),
                post,
            };
            Ok(HttpResponse::Ok().json(post_message))
        }
        Err(e) => {
            println!("{}", e);
            let error_message = Message {
                message: "Failed to get post".to_string(),
            };
            Ok(HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .json(error_message))
        }
    }
}

//#[get("/recent")]
//pub async fn get_recent_posts(data: web::Query<>)

#[get("/pinned")]
pub async fn get_today_pinned() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let posts: QueryResult<Vec<Post>> = posts::table.order(posts::likes.desc()).load(connection);
    if posts.is_err() {
        let posts_error_message = Message {
            message: "Failed to get posts".to_string(),
        };
        return Ok(HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .json(posts_error_message));
    }

    let posts_unwrap = posts.unwrap();

    let dt = Local::now();
    let naive_utc = dt.naive_utc();
    let offset = dt.offset().clone();
    let _dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);

    for post in posts_unwrap {
        let _date_posted: iso8601::DateTime = iso8601::datetime(post.posted.as_str()).unwrap();
        // TODO: purge posts older than a day
    }

    Ok(HttpResponse::Ok().body("test"))
}

#[get("/search")]
pub async fn search_posts(query: web::Query<SearchPostsDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let mut posts_vec: Vec<Post> = Vec::new();

    if query.post_id != 0 {
        let post_result: QueryResult<Post> = posts::table
            .find(query.post_id)
            .first::<Post>(connection);
        if post_result.is_ok() {
            let post = post_result.unwrap();
            posts_vec.push(post);
        }   
    }

    let all_posts_result: QueryResult<Vec<Post>> = posts::table
        .load(connection);

    if all_posts_result.is_ok() {
        let all_posts = all_posts_result.unwrap();
        let mut index = 0;
        for post in all_posts {
            if index > 15 {
                break;
            }
            if post.title.contains(query.name.as_str()) {
                posts_vec.push(post);
            }
            index += 1;
        }
    }

    Ok(HttpResponse::Ok().json(SearchPostsMessages { message: "Fetched posts".to_string(), posts: posts_vec }))
}
