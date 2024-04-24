use axum::extract::Request;
use tokio::net::TcpListener;
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;
use tower::Service;
use api_in_stress::app;
use api_in_stress::telemetry::{get_subscriber, init_subscriber};

async fn handle_connection(socket: tokio::net::TcpStream, app: axum::Router) {
    let socket = TokioIo::new(socket);
    let tower_service = app.clone();

    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
        tower_service.clone().call(request)
    });

    if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
    .serve_connection(socket, hyper_service)
        .await
    {
        eprintln!("failed to serve connection: {}", err);
    }
}

async fn serve(listener: TcpListener, app: axum::Router) {
    println!("Listening on: {}", listener.local_addr().unwrap());

    loop {
        let (socket, _remote_addr) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(socket, app.clone()));
    }
}

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("api_in_stress".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let app = app::create_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());

    serve(listener, app).await;
}