use crate::{app_state::AppState, models::Warrior, utilities::internal_error};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json;

use super::queries::GET_WARRIOR;

pub async fn get_warrior(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    if let Some(redis_warrior) = state.redis_store.get(&user_id).await {
        let warrior: Warrior = serde_json::from_str(&redis_warrior).unwrap();
        return (StatusCode::OK, headers, Json(warrior));
    }

    match sqlx::query_as(&GET_WARRIOR)
        .bind(&user_id)
        .fetch_optional(&state.database.primary_pool)
        .await
    {
        Ok(Some(warrior)) => {
            let warrior_json: String = serde_json::to_string(&warrior).unwrap();
            let _ = state.redis_store.set(&user_id, warrior_json).await;
            (StatusCode::OK, headers, Json(warrior))
        }
        Ok(None) => {
            let no_warrior = Warrior {
                id: "".to_string(),
                name: "".to_string(),
                dob: "".to_string(),
                fight_skills: None,
            };
            (StatusCode::NOT_FOUND, headers, Json(no_warrior))
        }
        Err(err) => {
            let _ = internal_error(err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                Json(Warrior {
                    id: "".to_string(),
                    name: "".to_string(),
                    dob: "".to_string(),
                    fight_skills: None,
                }),
            )
        }
    }
}
