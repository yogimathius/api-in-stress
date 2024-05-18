use super::queries::SEARCH_WARRIORS;
use crate::app_state::AppState;
use crate::models::Warrior;
use crate::utilities::internal_error;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json;
use std::collections::HashMap;

pub async fn search_warriors(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    if let Some(t) = params.get("t") {
        let query_key = format!("warriors:{}", t);
        if let Some(warriors_json) = state.redis_store.get(&query_key).await {
            let warriors: Vec<Warrior> =
                serde_json::from_str(&warriors_json).unwrap_or_else(|_| Vec::new());
            return (StatusCode::OK, headers, Json(warriors));
        } else {
            let warriors: Vec<Warrior> = sqlx::query_as(SEARCH_WARRIORS)
                .bind(t)
                .fetch_all(&state.database.primary_pool)
                .await
                .map_err(|err| internal_error(err))
                .unwrap_or_else(|_| Vec::new());

            let warriors_json = serde_json::to_string(&warriors).unwrap();
            let _ = state.redis_store.set(&query_key, warriors_json).await;

            return (StatusCode::OK, headers, Json(warriors));
        }
    } else {
        return (StatusCode::BAD_REQUEST, headers, Json(vec![]));
    }
}
