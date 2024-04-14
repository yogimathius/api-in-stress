use sqlx::postgres::PgPoolOptions;
mod database;
mod models;

use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Router,
    extract::Request

};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tower::Service;

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

    dotenv().ok();

    println!("DATABASE_URL: {:?}", std::env::var("DATABASE_URL"));
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(60000)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/warrior", post(create_warrior) )
        .route("/warrior/:id", get(get_warrior))
        .route("/warrior", get(search_warriors))
        .route("/counting-warriors", get(count_warriors))
        .with_state(pool)
        .layer(tower::ServiceBuilder::new().concurrency_limit(64))
        .layer(
            ServiceBuilder::new()
                // .layer(tower_http::trace::TraceLayer::new_for_http())
                // .layer(tower_http::compression::CompressionLayer::new())
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());

    loop {
        let (socket, _remote_addr) = listener.accept().await.unwrap();

        let tower_service = app.clone();

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);

            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                tower_service.clone().call(request)
            });

            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }}
