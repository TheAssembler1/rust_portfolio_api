use actix_web::{get, delete, post, put, App, HttpResponse, HttpServer, Responder, Result, web};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ConnConfig {
    port: u16,
    ip: String,
}

#[get("/")]
async fn check_health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
struct Test {
    message: String,
}

#[derive(Serialize)]
struct GetTest {
    id: String,
    message: String,
}

#[post("/test")]
async fn test_post(json: web::Json<Test>) -> impl Responder {
    println!("{:#?}", json);
    HttpResponse::Ok().finish()
}

#[put("/test/{test_id}")]
async fn test_put(path: web::Path<String>, json: web::Json<Test>) -> impl Responder {
    let test_id = path.into_inner();
    println!("{}", test_id);
    println!("{:#?}", json);
    HttpResponse::Ok().finish()
}

#[delete("/test/{test_id}")]
async fn test_delete(path: web::Path<String>) -> impl Responder {
    let test_id = path.into_inner();
    println!("{}", test_id);
    HttpResponse::Ok().finish()
}

#[get("/test/{test_id}")]
async fn test_get(path: web::Path<String>) -> Result<impl Responder> {
    let test_id = path.into_inner();
    let result = GetTest {
        id: test_id,
        message: String::from("testMessage"),
    };

    Ok(web::Json(result))
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
        .service(check_health)
        .service(test_post)
        .service(test_get)
        .service(test_delete)
        .service(test_put))
        .bind((config.ip, config.port))?
        .run()
        .await
}
