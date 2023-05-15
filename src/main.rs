use actix_web::{App, HttpServer};
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;

mod controllers;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ServerConfig {
    env: String,
    port: u16,
    ip: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DbConfig {
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

    println!("Attempting connection to database...");

    let pool = match Pool::new(&db_url[..]) {
        Ok(pool) => {
            println!("Connection to database successful!");
            pool
        }
        Err(error) => panic!("{:#?}", error),
    };

    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(r"SELECT 1").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(controllers::server_check::check_health)
            .service(controllers::test::test_post)
            .service(controllers::test::test_get)
            .service(controllers::test::test_delete)
            .service(controllers::test::test_put)
    })
    .bind((server_config.ip, server_config.port))?
    .run()
    .await
}
