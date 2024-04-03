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
    println!("Warrior created");
    // TODO - Error handling
    storage.create_warrior(payload).await
}

async fn get_warrior(storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warrior fetched");
    // TODO - Error handling
    Json(storage.get_warrior("1".to_string()).await)
}

async fn search_warriors(storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warriors searched");
    // TODO - Error handling
    // TODO - Implement search logic
    storage.search_warriors("".to_string()).await;
}

async fn count_warriors(storage: Extension<Storage>) -> impl IntoResponse {
    println!("Warriors counted");
    // TODO - Error handling
    storage.count_warriors().await;
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
