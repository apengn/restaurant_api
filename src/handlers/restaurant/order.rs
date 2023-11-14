use crate::handlers::restaurant::RestaurantAuthSession;
use axum::{http::StatusCode, response::IntoResponse};

pub async fn create(auth_session: RestaurantAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn list(auth_session: RestaurantAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get(auth_session: RestaurantAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
