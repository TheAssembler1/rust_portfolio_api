use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};
use crate::connection_pool;

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct Test {
    pub id: Option<u32>,
    pub message: String,
}

impl Test {
    pub fn post(&self) -> String {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        conn.exec_drop(r"INSERT INTO test (message) VALUES (?)", (self.message.clone(),)).unwrap();

        String::from("testMessage")
    }
}

#[post("/test")]
pub async fn test_post(json: web::Json<Test>) -> impl Responder {
    let id = json.into_inner().post();
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
    let result = Test {
        id: Some(0),
        message: String::from("testMessage"),
    };

    Ok(web::Json(result))
}
