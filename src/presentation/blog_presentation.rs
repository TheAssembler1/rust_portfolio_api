use actix_web::{put, get, post, Result, Responder, web::{Json, Query, Path}, delete};
use crate::{controller::blog_controller, model::{blog_model::{CreateBlog, UpdateBlog}, query::Pagination}};

#[post("/blogs")]
pub async fn create_blog(
    create_blog: Json<CreateBlog>
) -> Result<impl Responder> {
    Ok(blog_controller::create_blog(create_blog.into_inner()).await?)
}

#[get("/blogs")]
pub async fn get_blogs(pagination: Query<Pagination>) -> Result<impl Responder> {
    Ok(blog_controller::get_blogs(pagination.into_inner()).await?)
}

#[get("/blogs/{blog_id}")]
pub async fn get_blog(blog_id: Path<i32>) -> Result<impl Responder> {
    Ok(blog_controller::get_blog(blog_id.into_inner()).await?)
}

#[delete("/blogs/{blod_id}")]
pub async fn delete_blog(blog_id: Path<i32>) -> Result<impl Responder> {
    Ok(blog_controller::delete_blog(blog_id.into_inner()).await?)
}

#[put("/blogs/{blog_id}")]
pub async fn update_blog(blog_id: Path<i32>, blog_update: Json<UpdateBlog>) -> Result<impl Responder> {
    Ok(blog_controller::update_blog(blog_id.into_inner(),blog_update.into_inner()).await?)
}