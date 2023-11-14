use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Queryable)]
#[diesel(table_name = crate::schema::goods)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Goods {
    pub id: i32,
    pub restaurant_id: i32,
    pub goods_type_id: i32,
    pub price: f64,
    pub state: bool,
    pub img: String,
    pub name: String,
    pub info: String,
    pub count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::goods)]
pub struct NewGoods {
    #[serde(skip_deserializing)]
    pub restaurant_id: i32,
    pub goods_type_id: i32,
    pub price: f64,
    pub state: bool,
    pub img: String,
    pub name: String,
    pub info: String,
    pub count: i32,
}
