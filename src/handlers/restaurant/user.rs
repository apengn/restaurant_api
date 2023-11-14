use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::Pool,
    handlers::{
        internal_error,
        restaurant::{generate_md5, Credentials, RestaurantAuthSession},
    },
    model::restaurant::{NewRestaurant, Restaurant},
    schema,
};

pub async fn login(
    mut auth_session: RestaurantAuthSession,
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

pub async fn logout(mut auth_session: RestaurantAuthSession) -> impl IntoResponse {
    match auth_session.logout() {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn register(
    State(pool): State<Pool>,
    Json(mut new_restaurant): Json<NewRestaurant>,
) -> Result<Json<Restaurant>, (StatusCode, String)> {
    use schema::restaurants::dsl::*;
    let mut conn = pool.get().await.map_err(internal_error)?;

    new_restaurant.hashed_password = generate_md5(new_restaurant.hashed_password.as_str());

    let res = diesel::insert_into(restaurants)
        .values(new_restaurant)
        .returning(Restaurant::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn protected(auth_session: RestaurantAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_) => "protected".into_response(),

        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let paasswd = "123456";
        let md5str = md5::compute(paasswd);
        eprintln!(
            "e10adc3949ba59abbe56e057f20f883e == {}",
            format!("{:x}", md5str)
        );
    }
}
