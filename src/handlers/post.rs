use axum::{http::StatusCode, Json};

use crate::handlers::internal_error;
use crate::{
    db::Pool,
    model::post::{NewPost, Post},
    schema, DatabaseConnection,
};

use crate::handlers::Pagination;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn list_post(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let res = schema::posts::table
        .order(schema::posts::dsl::id.desc())
        .select(Post::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_posta(
    pagination: Option<Query<Pagination>>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let Query(pagination) = pagination.unwrap_or_default();
    let res = schema::posts::table
        .limit(pagination.page_size)
        .offset(pagination.offset())
        .order(schema::posts::dsl::id.desc())
        .select(Post::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    let count = schema::posts::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(internal_error)?;

    eprintln!("{count}");
    Ok(Json(res))
}

pub async fn create_post(
    State(pool): State<Pool>,
    Json(new_post): Json<NewPost>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let res = diesel::insert_into(crate::schema::posts::table)
        .values(new_post)
        .returning(Post::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn update_post(
    Path(id): Path<i32>,
    State(pool): State<Pool>,
    Json(new_post): Json<NewPost>,
) -> Result<(), (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    diesel::update(schema::posts::table.find(id))
        .set(schema::posts::dsl::body.eq(new_post.body))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(())
}

pub async fn delete_post(
    Path(id): Path<i32>,
    State(pool): State<Pool>,
) -> Result<(), (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    diesel::delete(schema::posts::dsl::posts.find(id))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(())
}
