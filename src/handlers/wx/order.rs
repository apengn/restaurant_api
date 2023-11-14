use crate::{
    db::Pool,
    handlers::{internal_error, wx::WXAuthSession, Pagination, PaginationReponse},
    model::order::{NewOrder, NewOrderParams, Order},
    schema,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn create(
    auth_session: WXAuthSession,
    State(pool): State<Pool>,
    Json(mut new_orderparams): Json<NewOrderParams>,
) -> Result<Json<()>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;

            let total_cost = new_orderparams
                .details
                .iter()
                .fold(0.0, |mut init, detail| {
                    init += detail.price;
                    init
                });

            let new_order = NewOrder {
                uuid: Uuid::new_v4().to_string(),
                restaurant_id: new_orderparams.restaurant_id,
                user_id: -1,
                wx_open_id: user.id,
                qrcode_location_id: new_orderparams.qrcode_location_id,
                state: "NO".to_string(),
                total_cost,
            };

            use schema::orders::dsl::{id as order_id, orders};
            use schema::orders_details::dsl::orders_details;

            conn.build_transaction()
                .read_committed()
                .run::<_, diesel::result::Error, _>(|mut conn| {
                    Box::pin(async move {
                        let order_id_v = diesel::insert_into(orders)
                            .values(&new_order)
                            .returning(order_id)
                            .get_result::<i32>(&mut conn)
                            .await?;

                        new_orderparams.details.iter_mut().for_each(|detail| {
                            detail.order_id = order_id_v;
                        });

                        let new_order_details =
                            new_orderparams
                                .details
                                .into_iter()
                                .fold(vec![], |mut init, detail| {
                                    init.push(detail);
                                    init
                                });

                        diesel::insert_into(orders_details)
                            .values(&new_order_details)
                            .execute(&mut conn)
                            .await?;

                        Ok(())
                    }) as _
                })
                .await
                .map_err(internal_error)?;

            Ok(Json(()))
        }
        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}

pub async fn list(
    auth_session: WXAuthSession,
    State(pool): State<Pool>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<PaginationReponse<Vec<Order>>>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;

            let Query(pagination) = pagination.unwrap_or_default();

            use schema::orders::dsl::*;
            let orders_v = orders
                .filter(wx_open_id.eq(user.id))
                .limit(pagination.page_size)
                .offset(pagination.offset())
                .order(id.desc())
                .select(orders::all_columns())
                .load::<Order>(&mut conn)
                .await
                .map_err(internal_error)?;

            let c = orders
                .filter(wx_open_id.eq(user.id))
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

pub async fn get(auth_session: WXAuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(_user) => "protected".into_response(),
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
