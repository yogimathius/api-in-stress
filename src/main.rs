mod database;
mod models;
mod schema;
mod seeds;

use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Router
};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tower::{timeout::TimeoutLayer, ServiceBuilder};

use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


use database::{create_warrior, get_warrior, search_warriors, count_warriors, handle_timeout_error};

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_diesel_async_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().expect(".env file not found");

    println!("DATABASE_URL: {:?}", std::env::var("DATABASE_URL"));
    let db_url = std::env::var("DATABASE_URL").unwrap();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    seeds::run_seeds(pool.clone()).await;

    let app = Router::new()
        .route("/warrior", post(create_warrior) )
        .route("/warrior/:id", get(get_warrior))
        .route("/warrior", get(search_warriors))
        .route("/counting-warriors", get(count_warriors))
        .with_state(pool)
        .layer(
            ServiceBuilder::new()
                // `timeout` will produce an error if the handler takes
                // too long so we must handle those
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
