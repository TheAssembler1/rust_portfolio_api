use crate::connection_pool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use anyhow;
use mysql::params;
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

impl DbTest {
    fn get_all() -> Result<Vec<Self>, anyhow::Error> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let mut results: Vec<Self> = Vec::new();

        let selected_tests = conn.query::<(u64, String), _>(r"SELECT * FROM test")?;

        for test in selected_tests {
            let (id, message) = test;
            results.push(Self { id, message });
        }

        Ok(results)
    }

    fn get(id: String) -> Result<HttpResponse, anyhow::Error> {
        let id: Result<u64, _> = id.parse();

        if id.is_err() {
            return Ok(HttpResponse::BadRequest().finish());
        }

        let id = id.unwrap();

        let mut conn = connection_pool::ConnectionPool::get_conn();
        let stmt = conn.prep("SELECT * FROM test WHERE id=:id")?;
        let result = conn.exec::<(u64, String), _, _>(
            stmt,
            params! {
                "id" => id
            },
        )?;
        let result = result.get(0);

        if result.is_none() {
            return Ok(HttpResponse::NotFound().finish());
        }

        let (id, message) = result.unwrap();

        Ok(HttpResponse::Ok().json(DbTest {
            id: *id,
            message: message.to_owned(),
        }))
    }

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
    let results = DbTest::get_all();

    if results.is_ok() {
        return HttpResponse::Ok().json(results.unwrap());
    }

    HttpResponse::InternalServerError().finish()
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
    let result = DbTest::get(path.into_inner());

    if result.is_ok() {
        return result.unwrap();
    }

    HttpResponse::InternalServerError().finish()
}
