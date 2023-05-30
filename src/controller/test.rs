use crate::connection_pool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
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
    fn post(test: Self) -> u64 {
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(r"INSERT INTO test (message) VALUES (?)", (test.message,))
            .unwrap();

        conn.last_insert_id()
    }

    fn put(id: String, test: Self) {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let id: u64 = id.parse().unwrap();

        conn.exec_drop(r"UPDATE test SET message=? WHERE id=?", (test.message, id))
            .unwrap();
    }
}

impl DbTest {
    fn get_all() -> Vec<Self> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let mut results: Vec<Self> = Vec::new();

        let selected_tests = conn
            .query::<(u64, String), _>(r"SELECT * FROM test")
            .unwrap();

        for test in selected_tests {
            let (id, message) = test;
            results.push(Self { id, message });
        }

        results
    }

    fn get(id: String) -> Self {
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

        Self {
            id: *id,
            message: result.to_owned(),
        }
    }

    fn delete(id: String) {
        let id: u64 = id.parse().unwrap();
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(r"DELETE FROM test WHERE id=?", (id,))
            .unwrap();
    }
}

#[post("/test")]
async fn test_post(json: web::Json<Test>) -> Result<impl Responder> {
    let id = Test::post(json.into_inner());
    Ok(web::Json(id))
}

#[get("/test")]
async fn test_get_all() -> Result<impl Responder> {
    let results = DbTest::get_all();
    Ok(web::Json(results))
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
async fn test_get(path: web::Path<String>) -> Result<impl Responder> {
    let result = DbTest::get(path.into_inner());
    Ok(web::Json(result))
}
