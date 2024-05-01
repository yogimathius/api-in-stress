use crate::app_state::AppState;
use crate::database::Database;
use crate::handlers::{
    count_warriors::count_warriors, create_warrior::create_warrior, get_warrior::get_warrior,
    search_warriors::search_warriors,
};
use crate::redis::RedisDatabase;
use crate::valid_fight_skills::DbFightSkills;
use axum::{
    http::Request,
    routing::{get, post},
    Router,
};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tokio::net::TcpListener;
use tower::Service;

pub struct Application {}

impl Application {
    pub async fn new() -> Router {
        let database = Database::new().await;
        let redis_store = RedisDatabase::new().await;
        let valid_skills = DbFightSkills::new(database.primary_pool.clone()).await;
        let debug = std::env::var("DEBUG").unwrap_or("false".to_string());
        if debug == "true" {
            println!("Debug mode enabled");
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .init();
        }
        let database_shard = std::env::var("SHARD").unwrap();

        let app_state = AppState {
            db_store: database.pool,
            primary_db_store: database.primary_pool,
            database_shard: database_shard,
            redis_store: redis_store,
            valid_skills: valid_skills,
        };

        Router::new()
            .route("/warrior", post(create_warrior))
            .route("/warrior/:id", get(get_warrior))
            .route("/warrior", get(search_warriors))
            .route("/counting-warriors", get(count_warriors))
            .with_state(app_state)
    }

    pub async fn serve(listener: TcpListener, app: axum::Router) {
        println!("Listening on: {}", listener.local_addr().unwrap());
        loop {
            let tower_service = app.clone();
            let (socket, _remote_addr) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let socket = TokioIo::new(socket);

                let hyper_service =
                    hyper::service::service_fn(move |request: Request<Incoming>| {
                        tower_service.clone().call(request)
                    });

                if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                    .serve_connection_with_upgrades(socket, hyper_service)
                    .await
                {
                    eprintln!("failed to serve connection: {err:#}");
                }
            });
        }
    }
}
