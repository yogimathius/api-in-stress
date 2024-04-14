use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Router,
};
use std::time::Duration;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use crate::handlers::{create_warrior, get_warrior, search_warriors, count_warriors, handle_timeout_error};
use crate::database::create_pool;

pub async fn create_app() -> Router {
    let pool = create_pool().await;
    Router::new()
        .route("/warrior", post(create_warrior) )
        .route("/warrior/:id", get(get_warrior))
        .route("/warrior", get(search_warriors))
        .route("/counting-warriors", get(count_warriors))
        .with_state(pool)
        .layer(tower::ServiceBuilder::new().concurrency_limit(64))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
        )
}