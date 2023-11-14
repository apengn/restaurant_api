// @generated automatically by Diesel CLI.

diesel::table! {
    goods (id) {
        id -> Int4,
        restaurant_id -> Int4,
        goods_type_id -> Int4,
        price -> Float8,
        state -> Bool,
        img -> Text,
        #[max_length = 120]
        name -> Varchar,
        info -> Text,
        count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    goods_type (id) {
        id -> Int4,
        restaurant_id -> Int4,
        #[max_length = 120]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        uuid -> Text,
        restaurant_id -> Int4,
        user_id -> Int4,
        wx_open_id -> Int4,
        qrcode_location_id -> Int4,
        state -> Text,
        total_cost -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orders_details (id) {
        id -> Int4,
        order_id -> Int4,
        good_id -> Int4,
        user_id -> Int4,
        wx_open_id -> Int4,
        count -> Nullable<Int4>,
        info -> Text,
        price -> Float8,
        goods_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    qrcode_location (id) {
        id -> Int4,
        restaurant_id -> Int4,
        number -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    restaurants (id) {
        id -> Int4,
        name -> Text,
        hashed_password -> Text,
        img -> Text,
        info -> Text,
        #[max_length = 11]
        phone -> Varchar,
        localtion -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wx_openid (id) {
        id -> Int4,
        #[max_length = 120]
        openid -> Varchar,
        #[max_length = 120]
        session_key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(goods -> goods_type (goods_type_id));
diesel::joinable!(goods -> restaurants (restaurant_id));
diesel::joinable!(goods_type -> restaurants (restaurant_id));
diesel::joinable!(orders -> qrcode_location (qrcode_location_id));
diesel::joinable!(orders -> restaurants (restaurant_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(orders -> wx_openid (wx_open_id));
diesel::joinable!(orders_details -> goods (good_id));
diesel::joinable!(orders_details -> orders (order_id));
diesel::joinable!(orders_details -> users (user_id));
diesel::joinable!(orders_details -> wx_openid (wx_open_id));
diesel::joinable!(qrcode_location -> restaurants (restaurant_id));

diesel::allow_tables_to_appear_in_same_query!(
    goods,
    goods_type,
    orders,
    orders_details,
    posts,
    qrcode_location,
    restaurants,
    users,
    wx_openid,
);
