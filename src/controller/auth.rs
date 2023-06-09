use crate::connection_pool;
use crate::jwt;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use bcrypt::hash_with_result;
use mysql::params;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DbUser {
    id: String,
    email: String,
    password: String,
    salt: String,
}

const BCRYPT_ITERATIONS: u32 = 12;

impl User {
    fn post(user: Self) -> String {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let id = Uuid::new_v4().to_string();
        let hash = hash_with_result(user.password, BCRYPT_ITERATIONS).unwrap();

        conn.exec_drop(
            r"INSERT INTO user (id, email, password, salt) VALUES (?, ?, ?, ?)",
            (&id, user.email, hash.to_string(), hash.get_salt()),
        )
        .unwrap();

        id
    }

    fn put(id: String, user: Self) {
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(
            r"UPDATE user SET email=?, password=? WHERE id=?",
            (user.email, user.password, id),
        )
        .unwrap();
    }

    fn login(user: Self) -> HttpResponse {
        let mut conn = connection_pool::ConnectionPool::get_conn();

        let pass_salt = conn
            .exec_first::<String, _, _>(
                r"SELECT password FROM user WHERE email=? LIMIT 1",
                (&user.email,),
            )
            .unwrap();

        if pass_salt == None {
            return HttpResponse::NotFound().finish();
        }

        let password = pass_salt.unwrap();

        if !bcrypt::verify(&user.password, &password).unwrap() {
            return HttpResponse::Unauthorized().finish();
        }

        HttpResponse::Ok().json(web::Json(jwt::jwt_create_token()))
    }

    fn validate_token(token: jwt::JwtToken) -> HttpResponse {
        if token.jwt_validate_token() {
            return HttpResponse::Unauthorized().finish();
        }

        HttpResponse::Ok().finish()
    }
}

impl DbUser {
    fn get_all() -> Vec<Self> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let mut results: Vec<Self> = Vec::new();

        let selected_tests = conn
            .query::<(String, String, String, String), _>(r"SELECT * FROM user")
            .unwrap();

        for test in selected_tests {
            let (id, email, password, salt) = test;
            results.push(Self {
                id,
                email,
                password,
                salt,
            });
        }

        results
    }

    fn get(id: String) -> Self {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let stmt = conn.prep("SELECT * FROM user WHERE id=:id").unwrap();
        let result = conn
            .exec::<(String, String, String, String), _, _>(
                stmt,
                params! {
                    "id" => id
                },
            )
            .unwrap();
        let result = result.get(0).unwrap();
        let (id, email, password, salt) = result;

        Self {
            id: id.to_owned(),
            email: email.to_owned(),
            password: password.to_owned(),
            salt: salt.to_owned(),
        }
    }

    fn delete(id: String) {
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(r"DELETE FROM user WHERE id=?", (id,))
            .unwrap();
    }
}

#[post("/auth/user")]
async fn user_post(json: web::Json<User>) -> Result<impl Responder> {
    let id = User::post(json.into_inner());
    Ok(web::Json(id))
}

#[get("/auth/user")]
async fn user_get_all() -> Result<impl Responder> {
    let results = DbUser::get_all();
    Ok(web::Json(results))
}

#[put("/auth/user/{user_id}")]
async fn user_put(path: web::Path<String>, json: web::Json<User>) -> impl Responder {
    User::put(path.into_inner(), json.into_inner());
    HttpResponse::Ok().finish()
}

#[delete("/auth/user/{user_id}")]
async fn user_delete(path: web::Path<String>) -> impl Responder {
    DbUser::delete(path.into_inner());
    HttpResponse::Ok().finish()
}

#[get("/auth/user/{user_id}")]
async fn user_get(path: web::Path<String>) -> Result<impl Responder> {
    let result = DbUser::get(path.into_inner());
    Ok(web::Json(result))
}

#[get("/auth/user-login")]
async fn user_login(json: web::Json<User>) -> impl Responder {
    User::login(json.into_inner())
}

#[get("/auth/validate-token")]
async fn user_validate_token(token: web::Query<jwt::JwtToken>) -> impl Responder {
    User::validate_token(token.into_inner())
}
