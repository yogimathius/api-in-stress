use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn create_primary_pool() -> sqlx::PgPool {
    let db_connection_str = "postgres://postgres:pass123@postgres:5432/warriors";

    PgPoolOptions::new()
        .max_connections(100000)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}
