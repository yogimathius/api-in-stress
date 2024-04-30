use crate::app_state::AppState;
use crate::database::Database;
use crate::handlers::{
    count_warriors::count_warriors, create_warrior::create_warrior, get_warrior::get_warrior,
    search_warriors::search_warriors,
};
use crate::redis::RedisDatabase;
use crate::utilities::handle_timeout_error;
use crate::valid_fight_skills::ValidFightSkills;
use axum::{
    error_handling::HandleErrorLayer,
    http::Request,
    routing::{get, post},
    Router,
};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::Service;
use tower::{timeout::TimeoutLayer, ServiceBuilder};

pub struct Application {}

impl Application {
    pub async fn new() -> Router {
        let database = Database::new().await;
        let redis_store = RedisDatabase::new().await;
        let valid_skills = ValidFightSkills::new(database.primary_pool.clone()).await;
        let debug = std::env::var("DEBUG").unwrap_or("false".to_string());
        if debug == "true" {
            println!("Debug mode enabled");
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .init();
        }

        let app_state = AppState {
            db_store: database.pool,
            primary_db_store: database.primary_pool,
            redis_store: redis_store,
            valid_skills: valid_skills,
        };

        Router::new()
            .route("/warrior", post(create_warrior))
            .route("/warrior/:id", get(get_warrior))
            .route("/warrior", get(search_warriors))
            .route("/counting-warriors", get(count_warriors))
            .with_state(app_state)
            .layer(tower::ServiceBuilder::new().concurrency_limit(2000))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .layer(TimeoutLayer::new(Duration::from_secs(30))),
            )
        // .layer(TraceLayer::new_for_http())
    }

    pub async fn serve(listener: TcpListener, app: axum::Router) {
        println!("Listening on: {}", listener.local_addr().unwrap());
        loop {
            let tower_service = app.clone();
            let (socket, _remote_addr) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
                // `TokioIo` converts between them.
                let socket = TokioIo::new(socket);

                // Hyper also has its own `Service` trait and doesn't use tower. We can use
                // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
                // `tower::Service::call`.
                let hyper_service =
                    hyper::service::service_fn(move |request: Request<Incoming>| {
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
                    .serve_connection_with_upgrades(socket, hyper_service)
                    .await
                {
                    eprintln!("failed to serve connection: {err:#}");
                }
            });
        }
    }
}
