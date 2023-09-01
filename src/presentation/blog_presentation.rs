use actix_web::{post, Result, Responder, HttpResponse};

#[post("/{blogs}")]
pub async fn create_blog() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}