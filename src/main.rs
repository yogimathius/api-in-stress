use api_in_stress::app::Application;

#[tokio::main]
async fn main() {
    println!("Starting server...");
    let app = Application::new().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    Application::serve(listener, app).await;
}