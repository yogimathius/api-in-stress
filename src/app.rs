use crate::app_state::AppState;
use axum::{
    error_handling::HandleErrorLayer, extract::MatchedPath, http::Request, routing::{get, post}, Router
};
use tower_http::trace::TraceLayer;
use std::time::Duration;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use crate::handlers::{create_warrior::create_warrior, get_warrior::get_warrior, search_warriors::search_warriors, count_warriors::count_warriors};
use crate::database::Database;
use crate::redis::RedisDatabase;
use crate::utilities::handle_timeout_error;
use crate::valid_fight_skills::ValidFightSkills;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Application {
    // port: u16,
}

impl Application {
    pub async fn new() -> Router {
        let database = Database::new().await;
        let redis_store = RedisDatabase::new().await;
        let valid_skills = ValidFightSkills::new();
        tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

        let app_state = AppState {
            db_store: database.pool,
            primary_db_store: database.primary_pool,
            redis_store: redis_store,
            valid_skills: valid_skills,
        };

        Router::new()
            .route("/warrior", post(create_warrior) )
            .route("/warrior/:id", get(get_warrior))
            .route("/warrior", get(search_warriors))
            .route("/counting-warriors", get(count_warriors))
            .with_state(app_state)
            .layer(tower::ServiceBuilder::new().concurrency_limit(200))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .layer(TimeoutLayer::new(Duration::from_secs(30)))
            )
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        // Log the matched route's path (with placeholders not filled in).
                        // Use request.uri() or OriginalUri if you want the real path.
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        info_span!(
                            "http_request",
                            method = ?request.method(),
                            matched_path,
                            some_other_field = tracing::field::Empty,
                        )
                    })
            )
    }

}