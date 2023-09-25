use crate::{
    controller::blog_controller,
    infrastructure::database::DbPool,
    model::{
        blog_model::{CreateBlog, UpdateBlog},
        jwt_model::AuthenticatedClaims,
        query::Pagination,
    },
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query, ReqData},
    Responder, Result,
};

#[post("/blogs")]
pub async fn create_blog(
    pool: Data<DbPool>,
    _authenticated_claims: ReqData<AuthenticatedClaims>,
    create_blog: Json<CreateBlog>,
) -> Result<impl Responder> {
    blog_controller::create_blog(pool.into_inner(), create_blog.into_inner()).await
}

#[get("/blogs")]
pub async fn get_blogs(
    pool: Data<DbPool>,
    pagination: Query<Pagination>,
) -> Result<impl Responder> {
    blog_controller::get_blogs(pool.into_inner(), pagination.into_inner()).await
}

#[get("/blogs/{blog_id}")]
pub async fn get_blog(pool: Data<DbPool>, blog_id: Path<i32>) -> Result<impl Responder> {
    blog_controller::get_blog(pool.into_inner(), blog_id.into_inner()).await
}

#[delete("/blogs/{blod_id}")]
pub async fn delete_blog(
    pool: Data<DbPool>,
    _authenticated_claims: ReqData<AuthenticatedClaims>,
    blog_id: Path<i32>,
) -> Result<impl Responder> {
    blog_controller::delete_blog(pool.into_inner(), blog_id.into_inner()).await
}

#[put("/blogs/{blog_id}")]
pub async fn update_blog(
    pool: Data<DbPool>,
    _authenticated_claims: ReqData<AuthenticatedClaims>,
    blog_id: Path<i32>,
    blog_update: Json<UpdateBlog>,
) -> Result<impl Responder> {
    blog_controller::update_blog(
        pool.into_inner(),
        blog_id.into_inner(),
        blog_update.into_inner(),
    )
    .await
}
