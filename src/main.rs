use actix_web::{HttpServer, App};
use dotenv::dotenv;
use serde::Deserialize;

mod controllers;

#[derive(Deserialize)]
pub struct ConnConfig {
    pub port: u16,
    pub ip: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = match envy::prefixed("SERVER_").from_env::<ConnConfig>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    println!("Server listening at http://{}:{}", config.ip, config.port);
    
    HttpServer::new(|| App::new()
        .service(controllers::server_check::check_health)
        .service(controllers::test::test_post)
        .service(controllers::test::test_get)
        .service(controllers::test::test_delete)
        .service(controllers::test::test_put))
        .bind((config.ip, config.port))?
        .run()
        .await
}
