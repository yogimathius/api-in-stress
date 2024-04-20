use axum::http::StatusCode;
use std::time::SystemTime;
use tower::BoxError;

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

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn report_time(start: SystemTime, action: &str) {
    match start.elapsed() {
        Ok(elapsed) => {
            println!("SystemTime taken to {:?}: {:?}", action, elapsed);
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}
