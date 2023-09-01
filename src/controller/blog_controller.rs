use diesel::QueryDsl;

use crate::model::blog_model::CreateBlog;
use crate::schema::blogs::dsl::blogs;

pub async fn create_blog(create_blog: CreateBlog<'_>) {

}