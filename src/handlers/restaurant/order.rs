use crate::db::Pool;
use crate::handlers::restaurant::RestaurantAuthSession;
use crate::handlers::{internal_error, Pagination, PaginationReponse};
use crate::model::order::{Order, UpdateOrderStateParams};
use crate::schema;
use axum::extract::{Path, Query, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn create(auth_session: RestaurantAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),

        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn list(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    state: Option<Path<i32>>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<PaginationReponse<Vec<Order>>>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;

            let Query(pagination) = pagination.unwrap_or_default();

            use schema::orders::dsl::{created_at, orders, restaurant_id, state as order_state};
            let orders_v = orders
                .filter(restaurant_id.eq(user.id))
                .filter(order_state.eq(state.as_ref().unwrap().0))
                .limit(pagination.page_size)
                .offset(pagination.offset())
                .order(created_at.desc())
                .select(orders::all_columns())
                .load::<Order>(&mut conn)
                .await
                .map_err(internal_error)?;

            let c = orders
                .filter(restaurant_id.eq(user.id))
                .filter(order_state.eq(state.unwrap().0))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .map_err(internal_error)?;

            let pagination_response = PaginationReponse {
                data: orders_v,
                current_page: pagination.page,
                page_size: pagination.page_size,
                total: c,
            };

            Ok(Json(pagination_response))
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}

pub async fn put(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    order_state_param: Option<Query<UpdateOrderStateParams>>,
) -> Result<(), (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;

            use schema::orders::dsl::*;
            let Query(order_state_param) = order_state_param.unwrap();

            diesel::update(
                orders
                    .filter(id.eq(order_state_param.id))
                    .filter(restaurant_id.eq(user.id)),
            )
            .set(state.eq(order_state_param.state))
            .execute(&mut conn)
            .await
            .map_err(internal_error)?;

            Ok(())
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}
