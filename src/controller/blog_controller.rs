use std::sync::Arc;

use actix_web::{HttpResponse, Responder};
use chrono::NaiveDateTime;
use log::error;

use diesel::result::{DatabaseErrorKind, Error as diesel_error};
use diesel::{self, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

use crate::infrastructure::api_error;
use crate::infrastructure::api_error::ApiError;
use crate::infrastructure::database::DbPool;
use crate::model::blog_model::{Blog, CreateBlog, UpdateBlog};
use crate::model::query::Pagination;
use crate::schema::blogs::dsl::blogs;
use crate::schema::blogs::{self as blogs_fields};

pub async fn create_blog(
    pool: Arc<DbPool>,
    create_blog: CreateBlog,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let blog_id = match diesel::insert_into(blogs)
        .values(create_blog)
        .execute(database_connection)
        .await
    {
        Ok(blog_id) => blog_id,
        Err(err) => {
            error!("{err}");

            if let diesel_error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) = err {
                return Ok(HttpResponse::Conflict().finish());
            }

            return Err(api_error::ApiError::DbError {
                message: "create_blog failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Created().body(blog_id.to_string()))
}

pub async fn get_blogs(
    pool: Arc<DbPool>,
    pagination: Pagination,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let blogs_results: Vec<(i32, NaiveDateTime, String, String)> = match blogs
        .select((
            blogs_fields::id,
            blogs_fields::created_at,
            blogs_fields::title,
            blogs_fields::summary,
        ))
        .order(blogs_fields::created_at.desc())
        .offset(pagination.page * pagination.page_size)
        .limit(pagination.page_size)
        .load(database_connection)
        .await
    {
        Ok(blogs_results) => blogs_results,
        Err(err) => {
            error!("{err}");

            return Err(api_error::ApiError::DbError {
                message: "get_blogs failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Created().json(blogs_results))
}

pub async fn get_blog(pool: Arc<DbPool>, blog_id: i32) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;
    let blog_result: Blog = match blogs.find(blog_id).first(database_connection).await {
        Ok(blog_result) => blog_result,
        Err(err) => {
            error!("{err}");

            if err == diesel_error::NotFound {
                return Ok(HttpResponse::NotFound().finish());
            }

            return Err(api_error::ApiError::DbError {
                message: "get_blog failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Ok().json(blog_result))
}

pub async fn delete_blog(pool: Arc<DbPool>, blog_id: i32) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;

    let num_rows = match diesel::delete(blogs.find(blog_id))
        .execute(database_connection)
        .await
    {
        Ok(num_rows) => num_rows,
        Err(err) => {
            error!("{err}");

            return Err(api_error::ApiError::DbError {
                message: "delete_blog failed".to_string(),
            }
            .into());
        }
    };

    if num_rows == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}

pub async fn update_blog(
    pool: Arc<DbPool>,
    blog_id: i32,
    blog_update: UpdateBlog,
) -> actix_web::Result<impl Responder> {
    let database_connection = &mut pool.get().await.map_err(|_| ApiError::DbPoolError)?;

    let num_rows = match diesel::update(blogs.find(blog_id))
        .set(blog_update)
        .execute(database_connection)
        .await
    {
        Ok(num_rows) => num_rows,
        Err(err) => {
            error!("{err}");

            return Err(api_error::ApiError::DbError {
                message: "update_blog failed".to_string(),
            }
            .into());
        }
    };

    if num_rows == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
