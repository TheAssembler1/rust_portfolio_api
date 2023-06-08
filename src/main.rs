#![crate_name = "rust_portfolio_api"]

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use mysql::*;
use serde::Deserialize;

mod connection_pool;
mod controller;
mod jwt;
mod middleware;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ServerConfig {
    env: String,
    port: u16,
    ip: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    user: String,
    password: String,
    host: String,
    port: u16,
    db_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info"))
        .expect("failed to init logger");
    dotenv().ok();

    let server_config = match envy::prefixed("SERVER_").from_env::<ServerConfig>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };
    let db_config = match envy::prefixed("DATABASE_").from_env::<DbConfig>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.db_name
    );

    jwt::init_jwt_key();

    println!(
        "Server listening at http://{}:{}",
        server_config.ip, server_config.port
    );
    println!("Database connection string: {}", db_url);

    let pool = connection_pool::ConnectionPool::init(db_url);
    connection_pool::CONNECTION_POOL
        .set(connection_pool::ConnectionPool { pool })
        .unwrap();

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(controller::server_check::check_health)
            .service(controller::test::test_post)
            .service(controller::test::test_get)
            .service(controller::test::test_get_all)
            .service(controller::test::test_delete)
            .service(controller::test::test_put)
            .service(controller::blog::blog_post)
            .service(controller::blog::blog_get)
            .service(controller::blog::blog_get_all)
            .service(controller::blog::blog_delete)
            .service(controller::blog::blog_put)
            .service(controller::auth::user_post)
            .service(controller::auth::user_get)
            .service(controller::auth::user_get_all)
            .service(controller::auth::user_delete)
            .service(controller::auth::user_put)
            .service(controller::auth::user_login)
            .service(controller::auth::user_validate_token)
    })
    .bind((server_config.ip, server_config.port))?
    .run()
    .await
}
