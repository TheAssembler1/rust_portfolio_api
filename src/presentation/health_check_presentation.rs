use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;
use actix_web::Result;

#[get("/health-check")]
pub async fn health_check() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().finish())
}