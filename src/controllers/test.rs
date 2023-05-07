use serde::{Deserialize, Serialize};
use actix_web::{get, delete, post, put, HttpResponse, Responder, Result, web};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Test {
    pub message: String,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub struct GetTest {
    pub id: String,
    pub message: String,
}

#[post("/test")]
pub async fn test_post(json: web::Json<Test>) -> impl Responder {
    println!("{:#?}", json);
    HttpResponse::Ok().finish()
}

#[put("/test/{test_id}")]
pub async fn test_put(path: web::Path<String>, json: web::Json<Test>) -> impl Responder {
    let test_id = path.into_inner();
    println!("{}", test_id);
    println!("{:#?}", json);
    HttpResponse::Ok().finish()
}

#[delete("/test/{test_id}")]
pub async fn test_delete(path: web::Path<String>) -> impl Responder {
    let test_id = path.into_inner();
    println!("{}", test_id);
    HttpResponse::Ok().finish()
}

#[get("/test/{test_id}")]
pub async fn test_get(path: web::Path<String>) -> Result<impl Responder> {
    let test_id = path.into_inner();
    let result = GetTest {
        id: test_id,
        message: String::from("testMessage"),
    };

    Ok(web::Json(result))
}