use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: i64,
    pub page_size: i64,
}