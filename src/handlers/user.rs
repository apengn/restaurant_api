use std::fmt::Debug;

use axum::{async_trait, http::StatusCode, response::IntoResponse, Json};
use axum_login::{AuthnBackend, UserId};

use password_auth::verify_password;
use serde::Deserialize;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{db::Pool, model::user::User, schema};

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
    pool: Pool,
}

impl Backend {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = diesel::result::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use schema::users::dsl::*;

        let mut conn = self.pool.get().await.unwrap();

        let user = users
            .filter(username.eq(creds.username.clone()))
            .filter(hashed_password.eq(creds.password.clone()))
            .select(User::as_select())
            .get_result(&mut conn)
            .await?;

        if verify_password(creds.username, &user.username)
            .ok()
            .is_none()
        {
            return Ok(Some(user));
        }

        return Ok(None);
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let mut conn = self.pool.get().await.unwrap();
        let user = schema::users::table
            .find(user_id)
            .select(User::as_select())
            .get_result(&mut conn)
            .await?;

        Ok(Some(user))
    }
}

type AuthSession = axum_login::AuthSession<Backend>;

pub async fn login(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    StatusCode::OK.into_response()
}

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout() {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
