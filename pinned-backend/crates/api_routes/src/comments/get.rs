use actix_web::{get, Responder, web, HttpResponse};
use crate::comments::dto::GetCommentDTO;

#[get("")]
pub async fn get_comment(data: web::Query<GetCommentDTO>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok())   
}
