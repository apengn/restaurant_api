use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::Pool,
    handlers::{internal_error, Pagination, PaginationReponse},
    model::goods_type::{GoodsType, NewGoodsType},
    schema,
};

use crate::handlers::restaurant::RestaurantAuthSession;

pub async fn create(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    Json(mut new_goods_type): Json<NewGoodsType>,
) -> Result<Json<GoodsType>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            use schema::goods_type::dsl::*;
            new_goods_type.restaurant_id = user.id;
            let mut conn = pool.get().await.map_err(internal_error)?;
            let res = diesel::insert_into(goods_type)
                .values(new_goods_type)
                .returning(GoodsType::as_returning())
                .get_result(&mut conn)
                .await
                .map_err(internal_error)?;
            Ok(Json(res))
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}

pub async fn update(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    id: Option<Path<i32>>,
    Json(new_goods_type): Json<NewGoodsType>,
) -> Result<(), (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;
            use schema::goods_type::dsl::{goods_type, id as goodtype_id, name, restaurant_id};

            diesel::update(
                goods_type
                    .filter(goodtype_id.eq(id.unwrap().0))
                    .filter(restaurant_id.eq(user.id)),
            )
            .set(name.eq(new_goods_type.name))
            .execute(&mut conn)
            .await
            .map_err(internal_error)?;
            Ok(())
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}

pub async fn get(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    id: Option<Path<i32>>,
) -> Result<Json<GoodsType>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;
            use schema::goods_type::dsl::{goods_type, id as goodtype_id, restaurant_id};
            let res = goods_type
                .filter(goodtype_id.eq(id.unwrap().0))
                .filter(restaurant_id.eq(user.id))
                .select(GoodsType::as_select())
                .get_result(&mut conn)
                .await
                .map_err(internal_error)?;
            Ok(Json(res))
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}
pub async fn list(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<PaginationReponse<Vec<GoodsType>>>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;
            let Query(pagination) = pagination.unwrap_or_default();
            use schema::goods_type::dsl::*;
            let res = goods_type
                .filter(restaurant_id.eq(user.id))
                .limit(pagination.page_size)
                .offset(pagination.offset())
                .order(id.desc())
                .select(GoodsType::as_select())
                .load(&mut conn)
                .await
                .map_err(internal_error)?;

            let count = goods_type
                .filter(restaurant_id.eq(user.id))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .map_err(internal_error)?;

            let data = PaginationReponse {
                current_page: pagination.page,
                page_size: pagination.page_size,
                total: count,
                data: res,
            };
            Ok(Json(data))
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}

pub async fn delete(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    id: Option<Path<i32>>,
) -> Result<(), (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;
            use schema::goods_type::dsl::{goods_type, id as goodtype_id, restaurant_id};

            diesel::delete(
                goods_type
                    .filter(goodtype_id.eq(id.unwrap().0))
                    .filter(restaurant_id.eq(user.id)),
            )
            .execute(&mut conn)
            .await
            .map_err(internal_error)?;

            Ok(())
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}
