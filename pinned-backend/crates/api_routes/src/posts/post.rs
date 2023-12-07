use actix_web::{post, Responder, HttpResponse, get};

#[post("/")]
pub async fn create_new_post() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
pub async fn get_post() -> impl Responder {
    HttpResponse::Ok()
}
