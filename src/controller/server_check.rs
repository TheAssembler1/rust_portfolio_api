use actix_web::{get, Responder, HttpResponse};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ConnConfig {
    pub port: u16,
    pub ip: String,
}

#[get("/")]
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok().finish()
}