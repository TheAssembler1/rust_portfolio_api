use std::{thread, time};
use std::time::Duration;

use actix_web::web::Data;
use actix_web::{middleware::Logger, App, HttpServer};

mod controller;
mod infrastructure;
mod model;
mod presentation;
mod schema;

use env_logger::Env;

use infrastructure::env_setup;
use log::{info, error};
use presentation::blog_presentation;
use presentation::health_check_presentation;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;

pub async fn start_server() -> anyhow::Result<()> {
    env_setup::init_env()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("server application starting");

    let server_config = env_setup::get_server_config()?;
    let database_config = env_setup::get_database_config()?;

    info!("{:#?}", server_config);
    info!("{:#?}", database_config);

    let server_config_host = &server_config.host;
    let server_config_port = &server_config.port;
    let database_connection_string = &database_config.url;

    info!("starting db connection pool");
    
    let pool = loop {
        info!("attempting connection");

        let config =
            AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_connection_string);
        match Pool::builder(config).build() {
            Ok(pool) => break pool,
            Err(err) => {
                error!("{}", err);
                info!("reattempting connection in 2 seconds");
                thread::sleep(time::Duration::from_millis(200));
            }
        };
    };

    info!("starting http server");
    info!("server listening at {server_config_host}:{server_config_port}");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(health_check_presentation::health_check)
            .service(blog_presentation::create_blog)
            .service(blog_presentation::get_blogs)
            .service(blog_presentation::get_blog)
            .service(blog_presentation::delete_blog)
            .service(blog_presentation::update_blog)
    })
    .bind((server_config_host.to_owned(), server_config_port.to_owned()))?
    .run()
    .await?;

    Ok(())
}
