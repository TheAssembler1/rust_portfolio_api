use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn check_health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 8080;

    println!("Server listening on http://{}:{}", ip, port);

    HttpServer::new(|| App::new().service(check_health))
        .bind((ip, port))?
        .run()
        .await
}
