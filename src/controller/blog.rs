use crate::connection_pool;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct DbBlog {
    pub id: u64,
    pub title: String,
    pub html: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct Blog {
    pub title: String,
    pub html: String,
}

impl Blog {
    pub fn post(blog: Self) -> u64 {
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(
            r"INSERT INTO blog (title, html) VALUES (?, ?)",
            (blog.title.clone(), blog.html.clone()),
        )
        .unwrap();

        conn.last_insert_id()
    }

    pub fn put(id: String, blog: Self) {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let id: u64 = id.parse().unwrap();

        conn.exec_drop(
            r"UPDATE blog SET title=?, html=? WHERE id=?",
            (blog.title.clone(), blog.html.clone(), id),
        )
        .unwrap();
    }
}

impl DbBlog {
    pub fn get_all() -> Vec<DbBlog> {
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let mut results: Vec<DbBlog> = Vec::new();

        let selected_blogs = conn
            .query::<(u64, String, String), _>(r"SELECT * FROM blog")
            .unwrap();

        for blog in selected_blogs {
            let (id, title, html) = blog;
            results.push(DbBlog { id, title, html });
        }

        results
    }

    pub fn get(id: String) -> DbBlog {
        let id: u64 = id.parse().unwrap();
        let mut conn = connection_pool::ConnectionPool::get_conn();
        let stmt = conn.prep("SELECT * FROM blog WHERE id=:id").unwrap();
        let result = conn
            .exec::<(u64, String, String), _, _>(
                stmt,
                params! {
                    "id" => id
                },
            )
            .unwrap();
        let result = result.get(0).unwrap();
        let (id, title, html) = result;

        DbBlog {
            id: *id,
            title: (*title).to_owned(),
            html: (*html).to_owned(),
        }
    }

    pub fn delete(id: String) {
        let id: u64 = id.parse().unwrap();
        let mut conn = connection_pool::ConnectionPool::get_conn();

        conn.exec_drop(r"DELETE FROM blog WHERE id=?", (id,))
            .unwrap();
    }
}

#[post("/blog")]
pub async fn blog_post(json: web::Json<Blog>) -> Result<impl Responder> {
    let id = Blog::post(json.into_inner());
    Ok(web::Json(id))
}

#[get("/blog")]
pub async fn blog_get_all() -> Result<impl Responder> {
    let results = DbBlog::get_all();
    Ok(web::Json(results))
}

#[put("/blog/{blog_id}")]
pub async fn blog_put(path: web::Path<String>, json: web::Json<Blog>) -> impl Responder {
    Blog::put(path.into_inner(), json.into_inner());
    HttpResponse::Ok().finish()
}

#[delete("/blog/{blog_id}")]
pub async fn blog_delete(path: web::Path<String>) -> impl Responder {
    DbBlog::delete(path.into_inner());
    HttpResponse::Ok().finish()
}

#[get("/blog/{blog_id}")]
pub async fn blog_get(path: web::Path<String>) -> Result<impl Responder> {
    let result = DbBlog::get(path.into_inner());
    Ok(web::Json(result))
}
