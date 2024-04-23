use crate::app_state::AppState;
use serde_json;
use axum::{
    extract::{Path, State}, http::StatusCode, Json
};
use redis::AsyncCommands;

use crate::models::Warrior;
use crate::queries::GET_WARRIOR;
use crate::utilities::internal_error;

pub async fn get_warrior(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<Warrior>, (StatusCode, String)> {
    let user_id_clone = user_id.clone();
    let mut redis_conn: bb8::PooledConnection<'_, bb8_redis::RedisConnectionManager> = state.redis_store.get().await.unwrap();

    if let Ok(user_id) = redis_conn.get::<_, String>(&user_id.clone()).await {
        let warrior: Warrior = serde_json::from_str(&user_id).unwrap();
        // report_time(start, "get_warrior from cache");

        return Ok(Json(warrior));
    }        

    let warrior: Warrior = sqlx::query_as(&GET_WARRIOR)
        .bind(user_id)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;

    let warrior_json: String = serde_json::to_string(&warrior).unwrap();
    let _ = redis_conn.set::<_, String, ()>(user_id_clone, warrior_json).await.map_err(|err: redis::RedisError| {
        eprintln!("Failed to cache warrior: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });
    // println!("Warrior cached successfully");

    Ok(Json(warrior))
}