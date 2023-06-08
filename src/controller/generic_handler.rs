use crate::connection_pool;
use actix_web::{HttpResponse, Result};
use anyhow;
use mysql::prelude::Queryable;
use mysql::{params, prelude::FromRow, Params};
use serde::Serialize;

/// Returns result of ok otherwise returns Interal Server Error 500 and hides server side error
pub fn generic_handler_handle(result: Result<HttpResponse, anyhow::Error>) -> HttpResponse {
    if result.is_ok() {
        return result.unwrap();
    }

    HttpResponse::InternalServerError().finish()
}

/// Returns params for post/put queries
///
/// # Arguments
///
/// * `self` - used to construct params
///
/// Row must have an id column auto incremented not null
pub trait GetParams {
    fn get_params(self: Self) -> Params;
}

/// A generic handler for get/get_all/put/post/delete endpoints
pub trait HttpHandler {
    /// Tuple data structure of db tuple type
    type DbTupleType: FromRow + Serialize + Clone;
    /// Struct of db type
    type DbType: From<Self::DbTupleType> + Serialize;
    /// Struct of user type usually identical to db type without the id
    type UserType: GetParams;

    /// returns the table to be acted upon
    fn get_table_name() -> String;
    /// returns tuple struct of two strings representing query
    /// struct.0 represents what fields seperated by commas
    /// struct.1 represents ? seperated by commas
    /// ex. (String::from("field")), (String::from("?"))
    fn get_post_params_strings() -> (String, String);
    /// returns string representing query
    /// format is field equal question mark seperated by commas
    /// ex. String::from("field=?")
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
        let id: Result<u64, _> = id.parse();

        if id.is_err() {
            return Ok(HttpResponse::BadRequest().finish());
        }

        let id = id.unwrap();
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(
            format!("DELETE FROM {} WHERE id=?", Self::get_table_name()),
            (id,),
        )?;

        if conn.affected_rows() <= 0 {
            return Ok(HttpResponse::NotFound().finish());
        }

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

        Ok(HttpResponse::Created().json(conn.last_insert_id()))
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

        if conn.affected_rows() <= 0 {
            return Ok(HttpResponse::NotFound().finish());
        }

        Ok(HttpResponse::Ok().json(id))
    }
}
