use crate::connection_pool;
use actix_web::{HttpResponse, Result};
use anyhow;
use mysql::prelude::Queryable;
use mysql::{params, prelude::FromRow};
use serde::Serialize;

pub fn generic_handler_handle(result: Result<HttpResponse, anyhow::Error>) -> HttpResponse {
    if result.is_ok() {
        return result.unwrap();
    }

    HttpResponse::InternalServerError().finish()
}

pub trait GetTable {
    fn get_table_name() -> String;
}

pub trait HttpHandler: Serialize + GetTable + From<Self::DbType> {
    type DbType: FromRow + Serialize + Clone;

    fn get(id: String) -> Result<HttpResponse, anyhow::Error> {
        let id: Result<u64, _> = id.parse();

        if id.is_err() {
            return Ok(HttpResponse::BadRequest().finish());
        }

        let id = id.unwrap();

        let mut conn = connection_pool::ConnectionPool::get_conn();
        let stmt = conn.prep(format!(
            "SELECT * FROM {} WHERE id=:id",
            Self::get_table_name()
        ))?;
        let result = conn.exec::<Self::DbType, _, _>(
            stmt,
            params! {
                "id" => id
            },
        )?;
        let result = result.get(0);

        if result.is_none() {
            return Ok(HttpResponse::NotFound().finish());
        }

        let result = Self::from(result.unwrap().clone());

        Ok(HttpResponse::Ok().json(result))
    }

    fn get_all() -> Result<HttpResponse, anyhow::Error> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let mut results: Vec<Self> = Vec::new();

        let selected_tests =
            conn.query::<Self::DbType, _>(format!("SELECT * FROM {}", Self::get_table_name()))?;

        for test in selected_tests {
            results.push(Self::from(test));
        }

        Ok(HttpResponse::Ok().json(results))
    }
}
