use crate::db::Pool;
use crate::handlers::internal_error;
use crate::handlers::restaurant::RestaurantAuthSession;
use crate::model::order::{Order, OrderParamsResponse};
use crate::model::order_detail::OrderDetail;
use crate::schema;
use axum::extract::{Query, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn get(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    id: Option<Query<i32>>,
) -> Result<Json<OrderParamsResponse>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;
            use schema::orders::dsl::{id as oid, orders, wx_open_id};

            let order: Order = orders
                .filter(wx_open_id.eq(user.id))
                .filter(oid.eq(id.unwrap().0))
                .select(orders::all_columns())
                .get_result::<Order>(&mut conn)
                .await
                .map_err(internal_error)?;

            use schema::orders_details::dsl::{order_id, orders_details};
            let ods = orders_details
                .filter(order_id.eq(order.id))
                .select(orders_details::all_columns())
                .get_results::<OrderDetail>(&mut conn)
                .await
                .map_err(internal_error)?;

            let data = OrderParamsResponse {
                uuid: order.uuid,
                restaurant_id: order.restaurant_id,
                user_id: order.user_id,
                wx_open_id: order.wx_open_id,
                qrcode_location_id: order.qrcode_location_id,
                state: order.state,
                total_cost: order.total_cost,
                details: ods,
            };
            Ok(Json(data))
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}
