use crate::app_state::AppState;
use axum::{
    extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, Json
};

use crate::models::NewWarrior;
use crate::queries::CREATE_WARRIOR;
use crate::utilities::internal_error;

pub async fn create_warrior(
    State(state): State<AppState>,
    Json(warrior): Json<NewWarrior>
) -> impl IntoResponse {

    let warrior_id: (i32,) = sqlx::query_as(&CREATE_WARRIOR)
        .bind(&warrior.name)
        .bind(&warrior.dob)
        .bind(&warrior.skills)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))
        .unwrap();

    let location: String = format!("/name/{:?}", warrior_id.0);
    let mut headers = HeaderMap::new();
    headers.insert("location", location.parse().unwrap());

    // report_time(start, "create_warrior");
    
    (StatusCode::CREATED, headers)
}