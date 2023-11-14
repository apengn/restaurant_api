use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Queryable)]
#[diesel(table_name = crate::schema::orders_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderDetail {
    pub id: i32,
    pub order_id: i32,
    pub count: i32,
    pub goods_name: String,
    pub info: String,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::orders_details)]
pub struct NewOrderDetail {
    #[serde(skip_deserializing)]
    pub order_id: i32,
    pub count: i32,
    pub price: f64,
    pub goods_name: String,
    pub info: String,
}
