use crate::app_state::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json;

use crate::models::Warrior;

pub async fn get_warrior(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let redis_warrior = state
        .redis_store
        .get(&user_id.clone())
        .await
        .unwrap_or("".to_string());
    if !redis_warrior.is_empty() {
        let warrior: Warrior = serde_json::from_str(&redis_warrior).unwrap();
        return (StatusCode::OK, headers, Json(warrior));
    } else {
        let no_warrior: Warrior = Warrior {
            id: "".to_string(),
            name: "".to_string(),
            dob: "".to_string(),
            fight_skills: None,
        };
        return (StatusCode::NOT_FOUND, headers, Json(no_warrior));
    }
}
