use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::order_detail::{NewOrderDetail, OrderDetail};

#[derive(Debug, Deserialize, Serialize, Clone, Queryable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: i32,
    pub uuid: String,
    pub restaurant_id: i32,
    pub user_id: i32,
    pub wx_open_id: i32,
    pub qrcode_location_id: i32,
    pub state: String,
    pub total_cost: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder {
    pub uuid: String,
    pub restaurant_id: i32,
    #[serde(skip_deserializing)]
    pub user_id: i32,
    #[serde(skip_deserializing)]
    pub wx_open_id: i32,
    pub qrcode_location_id: i32,
    pub state: String,
    pub total_cost: f64,
}

#[derive(Deserialize)]
pub struct NewOrderParams {
    pub uuid: String,
    pub restaurant_id: i32,
    #[serde(skip_deserializing)]
    pub user_id: i32,
    #[serde(skip_deserializing)]
    pub wx_open_id: i32,
    pub qrcode_location_id: i32,
    pub state: String, // 0 已下单，1 制作中，2 已买单
    pub total_cost: f64,
    pub details: Vec<NewOrderDetail>,
}

#[derive(Deserialize, Serialize)]
pub struct OrderParamsResponse {
    pub uuid: String,
    pub restaurant_id: i32,
    #[serde(skip_deserializing)]
    pub user_id: i32,
    #[serde(skip_deserializing)]
    pub wx_open_id: i32,
    pub qrcode_location_id: i32,
    pub state: String,
    pub total_cost: f64,
    pub details: Vec<OrderDetail>,
}
