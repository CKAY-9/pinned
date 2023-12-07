use actix_web::{get, Responder, HttpResponse};

#[get("")]
pub async fn get_post() -> impl Responder {
    HttpResponse::Ok()
}
