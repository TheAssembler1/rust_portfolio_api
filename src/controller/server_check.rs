use actix_web::{get, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConnConfig {
    pub port: u16,
    pub ip: String,
}

#[get("/")]
async fn check_health() -> impl Responder {
    HttpResponse::Ok().finish()
}
