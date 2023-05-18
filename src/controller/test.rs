use crate::connection_pool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct DbTest {
    pub id: Option<u64>,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct Test {
    pub message: String,
}

impl Test {
    pub fn post(test: Self) -> u64 {
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(
            r"INSERT INTO test (message) VALUES (?)",
            (test.message.clone(),),
        )
        .unwrap();

        conn.last_insert_id()
    }

    pub fn put(id: String, test: Self) {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let id: u64 = id.parse().unwrap();

        conn.exec_drop(
            r"UPDATE test SET message=? WHERE id=?",
            (test.message.clone(), id),
        )
        .unwrap();
    }
}

impl DbTest {
    pub fn get_all() -> Vec<DbTest> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let mut results: Vec<DbTest> = Vec::new();

        let selected_tests = conn
            .query::<(u64, String), _>(r"SELECT * FROM test")
            .unwrap();

        for test in selected_tests {
            let (id, message) = test;
            results.push(DbTest {
                id: Some(id),
                message,
            });
        }

        results
    }

    pub fn get(id: String) -> DbTest {
        let id: u64 = id.parse().unwrap();
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let stmt = conn.prep("SELECT * FROM test WHERE id=:id").unwrap();
        let result = conn
            .exec::<(u64, String), _, _>(
                stmt,
                params! {
                    "id" => id
                },
            )
            .unwrap();
        let result = result.get(0).unwrap();
        let (id, result) = result;

        DbTest {
            id: Some(*id),
            message: result.to_owned(),
        }
    }

    pub fn delete(id: String) {
        let id: u64 = id.parse().unwrap();
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(r"DELETE FROM test WHERE id=?", (id,))
            .unwrap();
    }
}

#[post("/test")]
pub async fn test_post(json: web::Json<Test>) -> Result<impl Responder> {
    let id = Test::post(json.into_inner());
    Ok(web::Json(id))
}

#[get("/test")]
pub async fn test_get_all() -> Result<impl Responder> {
    let results = DbTest::get_all();
    Ok(web::Json(results))
}

#[put("/test/{test_id}")]
pub async fn test_put(path: web::Path<String>, json: web::Json<Test>) -> impl Responder {
    Test::put(path.into_inner(), json.into_inner());
    HttpResponse::Ok().finish()
}

#[delete("/test/{test_id}")]
pub async fn test_delete(path: web::Path<String>) -> impl Responder {
    DbTest::delete(path.into_inner());
    HttpResponse::Ok().finish()
}

#[get("/test/{test_id}")]
pub async fn test_get(path: web::Path<String>) -> Result<impl Responder> {
    let result = DbTest::get(path.into_inner());
    Ok(web::Json(result))
}
