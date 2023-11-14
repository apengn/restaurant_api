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
    model::goods::{Goods, NewGoods},
    schema,
};

use crate::handlers::restaurant::RestaurantAuthSession;

pub async fn create(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    Json(mut new_goods): Json<NewGoods>,
) -> Result<Json<Goods>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            use schema::goods::dsl::*;
            new_goods.restaurant_id = user.id;
            let mut conn = pool.get().await.map_err(internal_error)?;
            let res = diesel::insert_into(goods)
                .values(&new_goods)
                .returning(goods::all_columns())
                .get_result::<Goods>(&mut conn)
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
) -> Result<Json<PaginationReponse<Vec<Goods>>>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;

            use schema::goods::dsl::*;
            let Query(pagination) = pagination.unwrap_or_default();
            let res = goods
                .filter(restaurant_id.eq(user.id))
                .limit(pagination.page_size)
                .offset(pagination.offset())
                .order(id.desc())
                .select(goods::all_columns())
                .load::<Goods>(&mut conn)
                .await
                .map_err(internal_error)?;

            let c = goods
                .filter(restaurant_id.eq(user.id))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .map_err(internal_error)?;

            let data = PaginationReponse {
                current_page: pagination.page,
                page_size: pagination.page_size,
                total: c,
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
            use schema::goods::dsl::{goods, id as goods_id, restaurant_id};

            diesel::delete(
                goods
                    .filter(goods_id.eq(id.unwrap().0))
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

pub async fn get(
    auth_session: RestaurantAuthSession,
    State(pool): State<Pool>,
    id: Option<Path<i32>>,
) -> Result<Json<Goods>, (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;
            use schema::goods::dsl::{goods, id as goods_id, restaurant_id};
            let res = goods
                .filter(goods_id.eq(id.unwrap().0))
                .filter(restaurant_id.eq(user.id))
                .select(goods::all_columns())
                .get_result::<Goods>(&mut conn)
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
    Json(new_goods): Json<NewGoods>,
) -> Result<(), (StatusCode, String)> {
    match auth_session.user {
        Some(user) => {
            let mut conn = pool.get().await.map_err(internal_error)?;
            use schema::goods::dsl::{goods, id as goods_id, restaurant_id};

            diesel::update(
                goods
                    .filter(goods_id.eq(id.unwrap().0))
                    .filter(restaurant_id.eq(user.id)),
            )
            .set(new_goods)
            .execute(&mut conn)
            .await
            .map_err(internal_error)?;
            Ok(())
        }

        None => Err((StatusCode::UNAUTHORIZED, "".to_string())),
    }
}
