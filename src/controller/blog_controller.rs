use actix_web::{HttpResponse, Responder};
use log::error;

use crate::infrastructure::{api_error, database};
use diesel::{self, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::model::blog_model::{Blog, CreateBlog};
use crate::schema::blogs::dsl::blogs;

pub async fn create_blog(create_blog: CreateBlog) -> actix_web::Result<impl Responder> {
    let database_connection = &mut database::establish_connection().await;
    let blog_id = match diesel::insert_into(blogs)
        .values(create_blog)
        .execute(database_connection)
        .await
    {
        Ok(blog_id) => blog_id,
        Err(err) => {
            error!("{err}");
            return Err(api_error::ApiError::DbError {
                message: "create_blog failed".to_string(),
            }
            .into());
        }
    };

    Ok(HttpResponse::Created().body(blog_id.to_string()))
}

pub async fn get_blogs() -> actix_web::Result<impl Responder> {
    /*let database_connection = &mut database::establish_connection().await;
    let blogs_results = blogs
        .select(Blog::as_select())
        .load(&mut database_connection)
        .await
        .unwrap();*/

    Ok(HttpResponse::Created().finish())
}
