use portfolio_api::start_server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    Ok(start_server().await?)
}   
