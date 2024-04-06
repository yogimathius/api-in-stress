use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Extension,
    Router
};
use tower::{timeout::TimeoutLayer, ServiceBuilder};

use std::time::Duration;

mod storage;
mod handlers;
use handlers::{create_warrior, get_warrior, search_warriors, count_warriors, handle_timeout_error};

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
