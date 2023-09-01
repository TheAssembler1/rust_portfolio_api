use diesel::{prelude::{Queryable, Insertable}, Selectable};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::blogs)]
pub struct Blog {
    pub id: usize,
    pub title: String,
    pub body: String,
    pub visible: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::blogs)]
pub struct CreateBlog<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub visible: bool,
}