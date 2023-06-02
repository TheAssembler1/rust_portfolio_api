use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use jwt_simple::prelude::*;
use mysql::*;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

mod connection_pool;
mod controller;

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

    // NOTE: logging init state
    println!("checking for jwt private key");

    // FIXME: put jwt file key init path in env var
    let jwt_private_key_file_path = Path::new(".jwt.private.key");
    let key: HS256Key;
    if jwt_private_key_file_path.exists() {
        println!("jwt private key found!");
        let mut file = File::open(jwt_private_key_file_path.to_str().unwrap())?;
        let mut buffer = Vec::new();
        let file_size = file.read_to_end(&mut buffer).unwrap();

        if file_size <= 0 {
            panic!("jwt private key file size was {}!", file_size);
        }

        key = HS256Key::from_bytes(&buffer);
    } else {
        println!("jwt private key not found!");
        println!("creating new jwt private key and writing to .private.key!");

        key = HS256Key::generate();

        let mut file = File::create(jwt_private_key_file_path.to_str().unwrap()).unwrap();
        file.write_all(&key.to_bytes()[..]).unwrap();
    }

    println!("{:#?}", server_config);
    println!("{:#?}", db_config);
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
    })
    .bind((server_config.ip, server_config.port))?
    .run()
    .await
}
