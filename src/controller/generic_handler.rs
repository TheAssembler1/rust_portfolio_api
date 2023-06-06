use crate::connection_pool;
use actix_web::{HttpResponse, Result};
use anyhow;
use mysql::prelude::Queryable;
use mysql::{params, prelude::FromRow, Params};
use serde::Serialize;

pub fn generic_handler_handle(result: Result<HttpResponse, anyhow::Error>) -> HttpResponse {
    if result.is_ok() {
        return result.unwrap();
    }

    HttpResponse::InternalServerError().finish()
}

pub trait GetParams {
    fn get_params(self: Self) -> Params;
}

pub trait HttpHandler {
    type DbTupleType: FromRow + Serialize + Clone;
    type DbType: From<Self::DbTupleType> + Serialize;
    type UserType: GetParams;

    fn get_table_name() -> String;
    fn get_post_params_strings() -> (String, String);
    fn get_put_params_string() -> String;

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
        let result = conn.exec::<Self::DbTupleType, _, _>(
            stmt,
            params! {
                "id" => id
            },
        )?;
        let result = result.get(0);

        if result.is_none() {
            return Ok(HttpResponse::NotFound().finish());
        }

        let result = Self::DbType::from(result.unwrap().clone());

        Ok(HttpResponse::Ok().json(result))
    }

    fn get_all() -> Result<HttpResponse, anyhow::Error> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let mut results: Vec<Self::DbType> = Vec::new();

        let selected_tests = conn
            .query::<Self::DbTupleType, _>(format!("SELECT * FROM {}", Self::get_table_name()))?;

        for test in selected_tests {
            results.push(Self::DbType::from(test));
        }

        Ok(HttpResponse::Ok().json(results))
    }

    fn delete(id: String) -> Result<HttpResponse, anyhow::Error> {
        let id: u64 = id.parse()?;
        let mut conn = connection_pool::ConnectionPool::get_conn();

        // FIXME: return 404 of not found
        conn.exec_drop(
            format!("DELETE FROM {} WHERE id=?", Self::get_table_name()),
            (id,),
        )?;

        Ok(HttpResponse::Ok().finish())
    }

    fn post(user_type: Self::UserType) -> Result<HttpResponse, anyhow::Error> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let param_strings = Self::get_post_params_strings();

        conn.exec_drop(
            format!(
                "INSERT INTO {} ({}) VALUES ({})",
                Self::get_table_name(),
                param_strings.0,
                param_strings.1,
            ),
            Self::UserType::get_params(user_type),
        )?;

        Ok(HttpResponse::Ok().json(conn.last_insert_id()))
    }

    fn put(id: String, user_type: Self::UserType) -> Result<HttpResponse, anyhow::Error> {
        let id: Result<u64, _> = id.parse();

        if id.is_err() {
            return Ok(HttpResponse::BadRequest().finish());
        }

        let id = id.unwrap();

        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(
            format!(
                "UPDATE {} SET {} WHERE id={}",
                Self::get_table_name(),
                Self::get_put_params_string(),
                id
            ),
            Self::UserType::get_params(user_type),
        )?;

        Ok(HttpResponse::Ok().json(id))
    }
}
