use axum::{
    error_handling::HandleErrorLayer, extract::{Json, Path, Query}, http::StatusCode, response::IntoResponse, routing::{get, post}, BoxError, Extension, Router
};
use tower::{timeout::TimeoutLayer, ServiceBuilder};

use std::collections::HashMap;
use std::time::Duration;

mod storage;

use storage::{Storage, Warrior};

async fn create_warrior(storage: Extension<Storage>, Json(payload): Json<Warrior>) -> impl IntoResponse {
    println!("Creating warrior: {:?}", payload);
    // TODO - Error handling
    (StatusCode::OK, [("x-foo", "bar")], Json(storage.create_warrior(payload).await))
}

async fn get_warrior(Path(user_id): Path<u32>, storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warrior fetched for id: {:?}", user_id);

    // TODO - Error handling
    (StatusCode::OK, [("x-foo", "bar")], Json(storage.get_warrior(user_id.to_string()).await))
}

async fn search_warriors(Query(params): Query<HashMap<String, String>>, storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warriors searched for: {:?}", params.get("term").unwrap_or(&"".to_string()));
    // TODO - Error handling
    // TODO - Implement search logic
    (StatusCode::OK, [("x-foo", "bar")], Json(storage.search_warriors("".to_string()).await))
}

async fn count_warriors(storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warriors counted");
    // TODO - Error handling
    let warrior_count = Json(storage.count_warriors().await);

    (StatusCode::OK, [("x-foo", "bar")], warrior_count)
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    println!("Error: {:?}", err);
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}

#[tokio::main]
async fn main() {
    let storage = storage::Storage::new();
    storage.initialize_data().await;

    let app = Router::new()
        .route("/warrior", post(create_warrior) )
        .route("/warrior/:id", get(get_warrior))
        .route("/warrior", get(search_warriors))
        .route("/counting-warriors", get(count_warriors))
        .layer(Extension(storage))
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
