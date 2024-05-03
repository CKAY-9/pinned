use crate::{
    dto::Message,
    posts::dto::{
        GetPostDTO,
        GetPostMessage,
        SearchPostsDTO,
        SearchPostsMessages,
        PostExploreMessage,
    },
};
use actix_web::{ get, web, HttpResponse, Responder };
use chrono::{ DateTime, Local };
use diesel::{ ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper };
use pinned_db::{create_connection, crud::posts::get_post_from_id};
use pinned_db_schema::{ models::Post, schema::posts };
use rand::{ seq::IteratorRandom, thread_rng };
use reqwest::StatusCode;

#[get("")]
pub async fn get_post(
    data: web::Query<GetPostDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let post = get_post_from_id(data.post_id);
    match post {
        Some(p) => {
            Ok(HttpResponse::Ok().json(GetPostMessage {
                message: "Fetched post".to_string(),
                post: p,
            }))
        },
        None => {
            Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                message: "Failed to get post".to_string(),
            }))
        }
    }
}

#[get("/explore")]
pub async fn get_explore_posts() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection: &mut diesel::prelude::PgConnection = &mut create_connection();
    let posts_result: QueryResult<Vec<Post>> = posts::table
        .select(Post::as_select())
        .load(connection);
    if posts_result.is_err() {
        return Ok(
            HttpResponse::Ok().json(Message {
                message: "Failed to get posts".to_string(),
            })
        );
    }

    let max_return = 10;
    let all_posts = posts_result.expect("Failed to get posts");

    if all_posts.iter().count() <= max_return {
        return Ok(
            HttpResponse::Ok().json(PostExploreMessage {
                message: "Got posts".to_string(),
                posts: all_posts,
            })
        );
    }

    let mut rng = thread_rng();
    let ps: Vec<Post> = all_posts.into_iter().choose_multiple(&mut rng, max_return);

    Ok(
        HttpResponse::Ok().json(PostExploreMessage {
            message: "Got posts".to_string(),
            posts: ps,
        })
    )
}

#[get("/pinned")]
pub async fn get_today_pinned() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let posts: QueryResult<Vec<Post>> = posts::table.order(posts::likes.desc()).load(connection);
    if posts.is_err() {
        let posts_error_message = Message {
            message: "Failed to get posts".to_string(),
        };
        return Ok(
            HttpResponse::Ok().status(StatusCode::INTERNAL_SERVER_ERROR).json(posts_error_message)
        );
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
pub async fn search_posts(
    query: web::Query<SearchPostsDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
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

    let all_posts_result: QueryResult<Vec<Post>> = posts::table.load(connection);

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

    Ok(
        HttpResponse::Ok().json(SearchPostsMessages {
            message: "Fetched posts".to_string(),
            posts: posts_vec,
        })
    )
}
