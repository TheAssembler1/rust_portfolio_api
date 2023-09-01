use actix_web::{HttpServer, App, middleware::Logger};

mod presentation;
mod model;
mod controller;
mod infrastructure;
mod schema;

use env_logger::Env;

use log::info;
use presentation::health_check_presentation as health_check_presentation;
use presentation::blog_presentation as blog_presentation;
use infrastructure::env_setup as env_setup;

pub async fn start_server() -> anyhow::Result<()> {
    env_setup::init_env()?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server_config = env_setup::get_server_config()?;
    let server_config_host = &server_config.host;
    let server_config_port = &server_config.port;

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .service(health_check_presentation::health_check)
        .service(blog_presentation::create_blog)
    })
    .bind((server_config_host.to_owned(), server_config_port.to_owned()))?
    .run()
    .await?;

    info!("server listening at {server_config_host}:{server_config_port}");

    Ok(())
}