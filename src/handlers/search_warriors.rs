use super::queries::{SEARCH_WARRIORS, SEARCH_WARRIORS_WITH_SKILLS};
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
    let query_key = format!("warriors:{:?}", params.get("t"));
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let no_warriors: Vec<Warrior> = vec![];
    if params.get("t").is_none() {
        return (StatusCode::BAD_REQUEST, headers, Json(no_warriors));
    }

    let warriors_json = state
        .redis_store
        .get(&query_key)
        .await
        .unwrap_or("".to_string());

    if !warriors_json.is_empty() {
        let warriors: Vec<Warrior> = serde_json::from_str(&warriors_json).unwrap();

        return (StatusCode::OK, headers, Json(warriors));
    } else {
        let skills_to_check = vec![params.get("t").unwrap().to_string()];
        let query = if state.valid_skills.are_valid_skills(&skills_to_check) {
            SEARCH_WARRIORS
        } else {
            SEARCH_WARRIORS_WITH_SKILLS
        };
        let warriors: Vec<Warrior> = sqlx::query_as(query)
            .bind(params.get("t"))
            .fetch_all(&state.primary_db_store)
            .await
            .map_err(|err| internal_error(err))
            .unwrap();

        let warriors_json = serde_json::to_string(&warriors).unwrap();
        let _ = state.redis_store.set(&query_key, warriors_json).await;

        return (StatusCode::OK, headers, Json(warriors));
    }
}
