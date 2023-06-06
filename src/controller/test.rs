use super::generic_handler::{generic_handler_handle, GetParams, HttpHandler};
use actix_web::{delete, get, post, put, web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct DbTest {
    id: u64,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct UserTest {
    message: String,
}

pub struct Test;

impl From<(u64, String)> for DbTest {
    fn from((id, message): (u64, String)) -> Self {
        Self { id, message }
    }
}

impl GetParams for UserTest {
    fn get_params(self: Self) -> mysql::Params {
        mysql::Params::Positional(Vec::from([mysql::Value::from(self.message.clone())]))
    }
}

impl HttpHandler for Test {
    type DbTupleType = (u64, String);
    type DbType = DbTest;
    type UserType = UserTest;

    fn get_table_name() -> String {
        String::from("test")
    }

    fn get_post_params_strings() -> (String, String) {
        (String::from("message"), String::from("?"))
    }

    fn get_put_params_string() -> String {
        String::from("message=?")
    }
}

#[get("/test")]
async fn test_get_all() -> impl Responder {
    generic_handler_handle(Test::get_all())
}

#[get("/test/{test_id}")]
async fn test_get(path: web::Path<String>) -> impl Responder {
    generic_handler_handle(Test::get(path.into_inner()))
}

#[post("/test")]
async fn test_post(json: web::Json<UserTest>) -> impl Responder {
    generic_handler_handle(Test::post(json.into_inner()))
}

#[put("/test/{test_id}")]
async fn test_put(path: web::Path<String>, json: web::Json<UserTest>) -> impl Responder {
    generic_handler_handle(Test::put(path.into_inner(), json.into_inner()))
}

#[delete("/test/{test_id}")]
async fn test_delete(path: web::Path<String>) -> impl Responder {
    generic_handler_handle(Test::delete(path.into_inner()))
}
