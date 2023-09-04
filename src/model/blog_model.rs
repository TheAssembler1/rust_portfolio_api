use chrono::NaiveDateTime;
use diesel::{prelude::{Queryable, Insertable, Selectable}, query_builder::AsChangeset};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::blogs)]
pub struct Blog {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub visible: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::blogs)]
pub struct CreateBlog {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub visible: bool,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::blogs)]
pub struct UpdateBlog {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub visible: bool,
}