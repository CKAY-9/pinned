use actix_web::{get, Responder, HttpResponse};

// Authentication
#[get("/discord")]
pub async fn discord_user_authentication() -> impl Responder {
   HttpResponse::Ok()
}

#[get("/github")]
pub async fn github_user_authentication() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/google")]
pub async fn google_user_authentication() -> impl Responder {
    HttpResponse::Ok()
}
