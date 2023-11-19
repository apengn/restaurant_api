use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use axum_login::AuthManagerLayer;

use crate::{
    db::Pool,
    handlers::{
        post::{create_post, delete_post, list_post, list_posta, update_post},
        restaurant::goods::{
            create as goods_create, delete as goods_delete, get as goods_get, list as goods_list,
            update as goods_update,
        },
        restaurant::goods_type::{
            create as goodstype_create, delete as goodstype_delete, get as goodstype_get,
            list as goodstype_list, update as goodstype_update,
        },
        restaurant::order::{list as order_list, put as order_update},
        restaurant::order_detail::get as get_order_detail,
        restaurant::user::{
            login as restaurant_login, logout as restaurant_logout,
            protected as restaurant_protected, register as restaurant_register,
        },
        restaurant::RestaurantBackend,
        user::{login, logout, protected, Backend},
        wx::order::{create as wx_create_order, list as wx_order_lis},
        wx::order_detail::get as get_wx_order_detail,
    },
};
use axum::BoxError;
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

pub async fn router_handlers(pg_pool: Pool, addr: std::net::SocketAddr) {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = Backend::new(pg_pool.clone());

    let auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayer::new(backend, session_layer));

    let restaurant_session_store = MemoryStore::default();
    let restaurant_session_layer = SessionManagerLayer::new(restaurant_session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let restaurant_backend = RestaurantBackend::new(pg_pool.clone());
    let restaurant_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayer::new(
            restaurant_backend,
            restaurant_session_layer,
        ));

    let user_app = Router::new()
        .route("/post/list", get(list_post))
        .route("/post/list_posta", get(list_posta))
        .route("/post", post(create_post))
        .route("/post/:id", delete(delete_post).put(update_post))
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/protected", get(protected))
        .layer(auth_service.clone())
        .with_state(pg_pool.clone());

    let wx_user_app = Router::new()
        .route("/wx/post/list", get(list_post))
        .route("/wx//post/list_posta", get(list_posta))
        .route("/wx//post", post(create_post))
        .route("/wx//post/:id", delete(delete_post).put(update_post))
        .route("/wx//login", post(login))
        .route("/wx//logout", get(logout))
        .route("/wx/order", get(get_wx_order_detail).post(wx_create_order))
        .route("/wx/orders", get(wx_order_lis))
        .layer(auth_service)
        .with_state(pg_pool.clone());

    let restaurant_app = Router::new()
        .route("/restaurant/register", post(restaurant_register))
        .route("/restaurant/login", post(restaurant_login))
        .route("/restaurant/protected", get(restaurant_protected))
        .route("/restaurant/logout", get(restaurant_logout))
        // goodstype
        .route("/restaurant/goodstype", post(goodstype_create))
        .route(
            "/restaurant/goodstype/:id",
            get(goodstype_get)
                .delete(goodstype_delete)
                .put(goodstype_update),
        )
        .route("/restaurant/goodstypes", get(goodstype_list))
        // goods
        .route("/restaurant/goods", post(goods_create))
        .route(
            "/restaurant/goods/:id",
            get(goods_get).delete(goods_delete).put(goods_update),
        )
        .route("/restaurant/goodsses", get(goods_list))
        .route("/restaurant/orders", get(order_list))
        .route("/restaurant/order/update", put(order_update))
        .route("/restaurant/order/", get(get_order_detail))
        .layer(restaurant_auth_service)
        .with_state(pg_pool);

    let app = Router::new()
        .merge(restaurant_app)
        .merge(user_app)
        .merge(wx_user_app);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
