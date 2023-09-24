use portfolio_api::start_server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    start_server().await
}   
