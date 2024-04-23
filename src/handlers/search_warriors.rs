use crate::app_state::AppState;
use serde_json;
use axum::{
    extract::{Query, State}, http::StatusCode, Json
};
use redis::AsyncCommands;

use crate::models::Warrior;
use super::queries::SEARCH_WARRIORS;
use crate::utilities::internal_error;
use std::collections::HashMap;

pub async fn search_warriors(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<Vec<Warrior>>, (StatusCode, String)> {
    let query_key = format!("warriors:{:?}", params.get("t"));
    println!("params: {:?}", params.get("t"));
    
    let mut redis_conn: bb8::PooledConnection<'_, bb8_redis::RedisConnectionManager> = state.redis_store.get().await.unwrap();
    if let Ok(warriors_json) = redis_conn.get::<_, String>(&query_key).await {
        let warriors: Vec<Warrior> = serde_json::from_str(&warriors_json).unwrap();

        return Ok(Json(warriors));
    }        

    let warriors:Vec<Warrior>  = sqlx::query_as(SEARCH_WARRIORS)
        .bind(params.get("t"))
        .fetch_all(&state.primary_db_store)
        .await
        .map_err(|err| internal_error(err))?;
    

    let warriors_json = serde_json::to_string(&warriors).unwrap();
    let _ = redis_conn.set::<_, String, ()>(&query_key, warriors_json).await.map_err(|err| {
        eprintln!("Failed to cache warriors: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    });
    println!("Warriors cached successfully");

    // report_time(start, "search_warriors");
    Ok(Json(warriors))
}