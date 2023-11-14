use crate::{db::Pool, model::restaurant::Restaurant, schema};
use axum::async_trait;
use axum_login::{AuthnBackend, UserId};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
pub mod goods;
pub mod goods_type;
pub mod order;
pub mod order_detail;
pub mod user;

pub type RestaurantAuthSession = axum_login::AuthSession<RestaurantBackend>;

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct RestaurantBackend {
    pool: Pool,
}

impl RestaurantBackend {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthnBackend for RestaurantBackend {
    type User = Restaurant;
    type Credentials = Credentials;
    type Error = diesel::result::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use schema::restaurants::dsl::*;

        let mut conn = self.pool.get().await.unwrap();

        let md5_passwd = generate_md5(&creds.password);
        let user = restaurants
            .filter(phone.eq(creds.phone))
            .filter(hashed_password.eq(md5_passwd))
            .select(Restaurant::as_select())
            .get_result(&mut conn)
            .await?;

        return Ok(Some(user));
    }

    async fn get_user(
        &self,
        restaurant_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        use schema::restaurants::dsl::*;

        let mut conn = self.pool.get().await.unwrap();
        let user = restaurants
            .find(restaurant_id)
            .select(Restaurant::as_select())
            .get_result(&mut conn)
            .await?;
        Ok(Some(user))
    }
}

pub fn generate_md5(data: &str) -> String {
    format!("{:x}", md5::compute(data))
}
