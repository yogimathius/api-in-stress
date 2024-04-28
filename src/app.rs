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

use crate::database::Database;
use crate::handlers::{
    count_warriors::count_warriors, create_warrior::create_warrior, get_warrior::get_warrior,
    search_warriors::search_warriors,
};
use crate::redis::RedisDatabase;
use crate::utilities::handle_timeout_error;
use crate::valid_fight_skills::ValidFightSkills;
use crate::{
    app_state::AppState,
    telemetry::{get_subscriber, init_subscriber},
};

pub struct Application {
    // port: u16,
}

impl Application {
    pub async fn new() -> Router {
        let database = Database::new().await;
        let redis_store = RedisDatabase::new().await;
        let valid_skills = ValidFightSkills::new();
        let debug = std::env::var("DEBUG").unwrap_or("false".to_string());
        if debug == "true" {
            println!("Debug mode enabled");
            let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
            init_subscriber(subscriber);
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
    }

    pub async fn serve(listener: TcpListener, app: axum::Router) {
        println!("Listening on: {}", listener.local_addr().unwrap());

        loop {
            let (socket, _remote_addr) = listener.accept().await.unwrap();
            tokio::spawn(handle_connection(socket, app.clone()));
        }
    }
}

async fn handle_connection(socket: tokio::net::TcpStream, app: axum::Router) {
    let socket = TokioIo::new(socket);
    // let tower_service = app.clone();

    let hyper_service =
        hyper::service::service_fn(move |request: Request<Incoming>| app.clone().call(request));

    if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
        .serve_connection(socket, hyper_service)
        .await
    {
        eprintln!("failed to serve connection: {}", err);
    }
}
