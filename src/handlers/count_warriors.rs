use crate::app_state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::Row;

use crate::utilities::internal_error;

pub async fn count_warriors(
    State(state): State<AppState>,
) -> Result<Json<i64>, (StatusCode, String)> {
    let query = "SELECT COUNT(*) FROM warriors;";

    let row = sqlx::query(query)
        .fetch_one(&state.database.primary_pool)
        .await
        .map_err(|err| internal_error(err))?;

    Ok(Json(row.get::<i64, _>(0)))
}
