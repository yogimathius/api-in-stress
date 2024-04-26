use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub struct Database {
    pub pool: sqlx::PgPool,
    pub primary_pool: sqlx::PgPool,
}

impl Database {
    pub async fn new() -> Self {
        let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

        let pool: sqlx::Pool<sqlx::Postgres> = create_pool(db_connection_str).await;

        let primary_db_connection_str = "postgres://postgres:pass123@postgres:5432/warriors".to_string();
        
        let primary_pool = create_pool(primary_db_connection_str).await;

        Self { pool, primary_pool }
    }
}

pub async fn create_pool(db_connection_str: String) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(10000)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}