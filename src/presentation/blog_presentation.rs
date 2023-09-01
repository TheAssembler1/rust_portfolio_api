use actix_web::{post, Result, Responder, web::Json};

use crate::{controller::blog_controller, model::blog_model::CreateBlog};

#[post("/{blogs}")]
pub async fn create_blog(
    create_blog: Json<CreateBlog>
) -> Result<impl Responder> {
    Ok(blog_controller::create_blog(create_blog.into_inner()).await?)
}