use super::generic_handler::{generic_handler_handle, GetTable, HttpHandler};
use crate::connection_pool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use anyhow;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct DbTest {
    id: u64,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct Test {
    message: String,
}

impl Test {
    fn post(test: Self) -> Result<u64, anyhow::Error> {
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(r"INSERT INTO test (message) VALUES (?)", (test.message,))?;

        Ok(conn.last_insert_id())
    }

    fn put(id: String, test: Self) -> Result<(), anyhow::Error> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let id: u64 = id.parse()?;

        conn.exec_drop(r"UPDATE test SET message=? WHERE id=?", (test.message, id))?;

        Ok(())
    }
}

impl GetTable for DbTest {
    fn get_table_name() -> String {
        String::from("test")
    }
}

impl From<(u64, String)> for DbTest {
    fn from((id, message): (u64, String)) -> Self {
        Self { id, message }
    }
}

impl HttpHandler for DbTest {
    type DbType = (u64, String);
}

impl DbTest {
    fn delete(id: String) -> Result<(), anyhow::Error> {
        let id: u64 = id.parse()?;
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(r"DELETE FROM test WHERE id=?", (id,))?;

        Ok(())
    }
}

#[post("/test")]
async fn test_post(json: web::Json<Test>) -> impl Responder {
    let result = Test::post(json.into_inner());

    if result.is_ok() {
        return HttpResponse::Ok().json(result.unwrap());
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/test")]
async fn test_get_all() -> impl Responder {
    generic_handler_handle(DbTest::get_all())
}

#[put("/test/{test_id}")]
async fn test_put(path: web::Path<String>, json: web::Json<Test>) -> impl Responder {
    Test::put(path.into_inner(), json.into_inner());
    HttpResponse::Ok().finish()
}

#[delete("/test/{test_id}")]
async fn test_delete(path: web::Path<String>) -> impl Responder {
    DbTest::delete(path.into_inner());
    HttpResponse::Ok().finish()
}

#[get("/test/{test_id}")]
async fn test_get(path: web::Path<String>) -> impl Responder {
    generic_handler_handle(DbTest::get(path.into_inner()))
}
