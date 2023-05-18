use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use std::{env, io};
use once_cell::unsync::OnceCell;

mod controller;
mod connection_pool;

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

    // NOTE: logging init state
    println!("{:#?}", server_config);
    println!("{:#?}", db_config);
    println!(
        "Server listening at http://{}:{}",
        server_config.ip, server_config.port
    );
    println!("Database connection string: {}", db_url);

    let pool = connection_pool::ConnectionPool::init(db_url);
    connection_pool::CONNECTION_POOL.set(connection_pool::ConnectionPool {
        pool
    }).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(controller::server_check::check_health)
            .service(controller::test::test_post)
            .service(controller::test::test_get)
            .service(controller::test::test_delete)
            .service(controller::test::test_put)
    })
    .bind((server_config.ip, server_config.port))?
    .run()
    .await
}
