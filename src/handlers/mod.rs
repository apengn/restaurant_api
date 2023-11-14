use axum::http::StatusCode;
pub mod post;

pub mod admin;
pub mod restaurant;
pub mod user;
pub mod wx;

pub mod wx_user;
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// The query parameters for todos index
#[derive(Debug, serde::Deserialize, Default)]
pub struct Pagination {
    pub page: i64,
    pub page_size: i64,
}

impl Pagination {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PaginationReponse<T> {
    pub current_page: i64,
    pub page_size: i64,
    pub total: i64,
    pub data: T,
}
