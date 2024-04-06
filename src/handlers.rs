
use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, Extension, Json};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use std::collections::HashMap;
use tower::BoxError;

use crate::storage::{Storage, Warrior};

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn create_warrior(storage: Extension<Storage>, Json(payload): Json<Warrior>) -> impl IntoResponse {
    println!("Creating warrior: {:?}", payload);
    // TODO - Error handling
    (StatusCode::OK, [("x-foo", "bar")], Json(storage.create_warrior(payload).await))
}

pub async fn get_warrior(Path(user_id): Path<u32>, storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warrior fetched for id: {:?}", user_id);

    // TODO - Error handling
    (StatusCode::OK, [("x-foo", "bar")], Json(storage.get_warrior(user_id.to_string()).await))
}

pub async fn search_warriors(Query(params): Query<HashMap<String, String>>, storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warriors searched for: {:?}", params.get("term").unwrap_or(&"".to_string()));
    // TODO - Error handling
    // TODO - Implement search logic
    (StatusCode::OK, [("x-foo", "bar")], Json(storage.search_warriors("".to_string()).await))
}

pub async fn count_warriors(storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warriors counted");
    // TODO - Error handling
    let warrior_count = Json(storage.count_warriors().await);

    (StatusCode::OK, [("x-foo", "bar")], warrior_count)
}

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
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
