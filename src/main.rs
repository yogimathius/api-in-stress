use std::sync::Arc;

use api_in_stress::app::Application;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let app = Application::new().await;
    let semaphore = Arc::new(Semaphore::new(1000));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    Application::serve(listener, app, semaphore).await;
}
