use crate::app_state::AppState;
use sqlx::Row;
use axum::{extract::State, http::StatusCode, Json};
use redis::AsyncCommands;
use std::time::SystemTime;

use crate::utilities::{report_time, internal_error};

pub async fn count_warriors(
    State(state): State<AppState>,
) -> Result<Json<i64>, (StatusCode, String)>{
    let start = SystemTime::now();

    println!("Warriors counted");
    // TODO - Error handling

    let query = "SELECT COUNT(*) FROM warriors;";

    let row = sqlx::query(query)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;

    report_time(start, "count_warriors");

    Ok(Json(row.get::<i64, _>(0)))
    
}