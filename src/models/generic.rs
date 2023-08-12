use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationData {
    page: i64,
    per_page: i64,
    pages: i64,
    total_count: i64,
}

impl PaginationData {
    pub fn new(page: i64, per_page: i64, total_count: i64) -> Self {
        let pages = ((total_count as f64 / per_page as f64).ceil()) as i64;

        PaginationData {
            page: page,
            per_page: per_page,
            pages: pages,
            total_count: total_count,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BacksonResponse<T: Serialize> {
    pub(crate) message: String,
    pub(crate) code: u16,
    pub(crate) data: Option<T>,
}
