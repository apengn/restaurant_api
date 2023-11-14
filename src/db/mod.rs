use diesel_async::pooled_connection::bb8;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use diesel_async::AsyncPgConnection;

pub type Pool = bb8::Pool<AsyncPgConnection>;

pub async fn establish_connection(connection_url: impl Into<String>) -> Pool {
    // create a new connection pool with the default config
    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(connection_url);
    let pool: bb8::Pool<diesel_async::AsyncPgConnection> =
        bb8::Pool::builder().build(config).await.unwrap();
    pool
}
