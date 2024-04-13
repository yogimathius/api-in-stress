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
use std::{sync::Arc, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tower::Service;
// use tokio::sync::Semaphore;

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
    // let db_url = std::env::var("DATABASE_URL").unwrap();
    // let mut conn = PgConnection::establish(&db_url).unwrap();
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());
    // let semaphore = Arc::new(Semaphore::new(10));

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
    // Continuously accept new connections.
    loop {
        // In this example we discard the remote address. See `fn serve_with_connect_info` for how
        // to expose that.
        let (socket, _remote_addr) = listener.accept().await.unwrap();

        // We don't need to call `poll_ready` because `Router` is always ready.
        let tower_service = app.clone();

        // Spawn a task to handle the connection. That way we can multiple connections
        // concurrently.
        tokio::spawn(async move {
            // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
            // `TokioIo` converts between them.
            let socket = TokioIo::new(socket);

            // Hyper also has its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
            // `tower::Service::call`.
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                // We have to clone `tower_service` because hyper's `Service` uses `&self` whereas
                // tower's `Service` requires `&mut self`.
                //
                // We don't need to call `poll_ready` since `Router` is always ready.
                tower_service.clone().call(request)
            });

            // `server::conn::auto::Builder` supports both http1 and http2.
            //
            // `TokioExecutor` tells hyper to use `tokio::spawn` to spawn tasks.
            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                // `serve_connection_with_upgrades` is required for websockets. If you don't need
                // that you can use `serve_connection` instead.
                .serve_connection(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }}
