use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::goods_type)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GoodsType {
    pub id: i32,
    pub name: String,
    pub restaurant_id: i32,
}

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::goods_type)]
pub struct NewGoodsType {
    pub name: String,
    #[serde(skip_deserializing)]
    pub restaurant_id: i32,
}
