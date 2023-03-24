use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConnConfig {
    port: u16,
    ip: String,
}

#[get("/")]
async fn check_health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = match envy::prefixed("SERVER_").from_env::<ConnConfig>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    println!("Server listening at http://{}:{}", config.ip, config.port);

    HttpServer::new(|| App::new().service(check_health))
        .bind((config.ip, config.port))?
        .run()
        .await
}
