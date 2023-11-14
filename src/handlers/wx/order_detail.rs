use crate::handlers::wx::WXAuthSession;
use axum::{http::StatusCode, response::IntoResponse};

pub async fn create(auth_session: WXAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn list(auth_session: WXAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get(auth_session: WXAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
