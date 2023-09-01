use actix_web::{Responder, HttpResponse};
use anyhow;

use crate::infrastructure::database;
use diesel;
use diesel_async::RunQueryDsl;

use crate::model::blog_model::CreateBlog;
use crate::schema::blogs::dsl::blogs;

pub async fn create_blog(create_blog: CreateBlog<'_>) -> anyhow::Result<impl Responder, BlogError> {
    let database_connection = &mut database::establish_connection().await;
    let blog_id = diesel::insert_into(blogs)
        .values(create_blog)
        .execute(database_connection)
        .await
        .unwrap();

    Ok(HttpResponse::Created().body(blog_id.to_string()))
}
