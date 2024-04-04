use axum::{
    Extension,
    extract::{Json, Query, Path},
    routing::get,
    routing::post,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use std::collections::HashMap;

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

#[tokio::main]
async fn main() {
    let storage = storage::Storage::new();
    storage.initialize_data().await;

    let app = Router::new()
        .route("/warrior", post(create_warrior) )
        .route("/warrior/:id", get(get_warrior))
        .route("/warrior", get(search_warriors))
        .route("/counting-warriors", get(count_warriors))
        .layer(Extension(storage));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
