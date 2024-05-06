use crate::app_state::AppState;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json;

use crate::models::Warrior;

use crate::utilities::internal_error;

use super::queries::GET_WARRIOR;

pub async fn get_warrior(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let user_id_clone = user_id.clone();
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
        let result: Option<Warrior> = sqlx::query_as(&GET_WARRIOR)
            .bind(user_id)
            .fetch_optional(&state.primary_db_store)
            .await
            .map_err(|err| internal_error(err))
            .unwrap();
        match result {
            Some(warrior) => {
                let warrior_json: String = serde_json::to_string(&warrior).unwrap();
                let _ = state.redis_store.set(&user_id_clone, warrior_json);

                return (StatusCode::OK, headers, Json(warrior));
            }
            None => {
                let no_warrior: Warrior = Warrior {
                    id: "".to_string(),
                    name: "".to_string(),
                    dob: "".to_string(),
                    fight_skills: "".to_string(),
                };
                return (StatusCode::NOT_FOUND, headers, Json(no_warrior));
            }
        }
    }
}
