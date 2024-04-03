use axum::{
    Extension,
    extract::Json,
    routing::get,
    routing::post,
    Router,
    response::IntoResponse
};
mod storage;

use storage::{Storage, Warrior};

async fn create_warrior(storage: Extension<Storage>, Json(payload): Json<Warrior>) -> &'static str {
    // Implement logic for creating a warrior
    println!("Warrior created");
    storage.create_warrior(payload).await
}

async fn get_warrior(storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warrior fetched");
    Json(storage.get_warrior("1".to_string()).await)
}

async fn search_warriors(storage: Extension<Storage>) -> &'static str {
    // Implement logic for searching warriors
    "Warriors searched"
}

async fn count_warriors(storage: Extension<Storage>) -> &'static str {
    // Implement logic for counting warriors
    "Warriors counted"
}

#[tokio::main]
async fn main() {
    let storage = storage::Storage::new();
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
