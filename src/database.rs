use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn create_pool() -> sqlx::PgPool {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    PgPoolOptions::new()
        .max_connections(60000)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}
