use crate::app_state::AppState;
use serde_json;
use axum::{
    extract::{Path, State}, http::{HeaderMap, StatusCode}, response::IntoResponse, Json
};
use redis::AsyncCommands;

use crate::models::Warrior;

use crate::utilities::internal_error;

use super::queries::GET_WARRIOR;

pub async fn get_warrior(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let user_id_clone = user_id.clone();
    let mut redis_conn: bb8::PooledConnection<'_, bb8_redis::RedisConnectionManager> = state.redis_store.get().await.unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    
    if let Ok(user_id) = redis_conn.get::<_, String>(&user_id.clone()).await {
        let warrior: Warrior = serde_json::from_str(&user_id).unwrap();

        return (StatusCode::OK, headers, Json(warrior));

    }        

    let result: Option<Warrior> = sqlx::query_as(&GET_WARRIOR)
        .bind(user_id)
        .fetch_optional(&state.primary_db_store)
        .await
        .map_err(|err| internal_error(err))
        .unwrap();


    match result {
        Some(warrior) => {
            let warrior_json: String = serde_json::to_string(&warrior).unwrap();
            let _ = redis_conn.set::<_, String, ()>(user_id_clone, warrior_json).await.map_err(|err: redis::RedisError| {
                eprintln!("Failed to cache warrior: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            });
            return (StatusCode::OK, headers, Json(warrior));
        },
        None => {
            let no_warrior: Warrior = Warrior {
                id: "".to_string(),
                name: "".to_string(),
                dob: "".to_string(),
                fight_skills: None
            };
            return (StatusCode::NOT_FOUND, headers, Json(no_warrior));
        }
    }
}