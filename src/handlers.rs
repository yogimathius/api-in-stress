use crate::app_state::AppState;
use serde_json;
use sqlx::Row;
use axum::{
    extract::{Path, Query, State}, http::{HeaderMap, StatusCode}, response::IntoResponse, Json
};
use redis::AsyncCommands;
use std::time::SystemTime;

use crate::models::{NewWarrior, Warrior};
use std::collections::HashMap;
use tower::BoxError;

pub async fn create_warrior(
    State(state): State<AppState>,
    Json(warrior): Json<NewWarrior>
) -> impl IntoResponse {
    let start = SystemTime::now();

    println!("Creating warrior: {:?}", warrior);

    let complete_query = r#"
        WITH inserted_warrior AS (
            INSERT INTO warriors (name, dob)
            VALUES ($1, $2)
            RETURNING id
        ),
        inserted_skills AS (
            INSERT INTO skills (name)
            SELECT skill_name
            FROM unnest(($3::text[])) AS skill_name
            ON CONFLICT (name) DO NOTHING
            RETURNING id, name
        ),
        inserted_warrior_skills AS (
            INSERT INTO warrior_skills (warrior_id, skill_id)
            SELECT inserted_warrior.id, COALESCE(existing_skills.id, new_skill.id)
            FROM inserted_warrior
            CROSS JOIN inserted_skills
            LEFT JOIN skills existing_skills ON existing_skills.name = inserted_skills.name
            LEFT JOIN (
                SELECT id, name
                FROM inserted_skills
            ) AS new_skill ON true
        )
        SELECT id from inserted_warrior;
    "#;

    let warrior_id: (i32,) = sqlx::query_as(&complete_query)
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

    match start.elapsed() {
        Ok(elapsed) => {
            println!("SystemTime taken to create warrior: {:?}", elapsed);
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
    
    (StatusCode::CREATED, headers)
}

pub async fn get_warrior(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Warrior>, (StatusCode, String)> {
    let start = SystemTime::now();

    println!("Warrior fetched for id: {:?}", user_id);
    // Try to fetch the result from Redis cache
    if let Ok(mut redis_conn) = state.redis_store.get().await {
        println!("Fetching warrior from cache {}", user_id);
        if let Ok(user_id) = redis_conn.get::<_, String>(&user_id).await.map_err(|err| {
            eprintln!("Failed to fetch warriors from cache: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }) {
            let warrior: Warrior = serde_json::from_str(&user_id).unwrap();
            return Ok(Json(warrior));
        }        
    }
    
    let query = format!(
        r#"SELECT warriors.*,
            (SELECT array_agg(skills.name)
                FROM skills
                INNER JOIN warrior_skills ON skills.id = warrior_skills.skill_id
                WHERE warrior_skills.warrior_id = warriors.id
            ) AS fight_skills
        FROM warriors
        WHERE id = {};"#,
        user_id
    );

    let warrior: Warrior = sqlx::query_as(&query)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;

    // Cache the result in Redis
    if let Ok(mut redis_conn) = state.redis_store.get().await {
        let warrior_json: String = serde_json::to_string(&warrior).unwrap();
        let _ = redis_conn.set::<_, String, ()>(&user_id, warrior_json).await.map_err(|err: redis::RedisError| {
            eprintln!("Failed to cache warrior: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
        println!("Warrior cached successfully");
    }
    match start.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("SystemTime taken to fetch warrior: {:?}", elapsed);
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    Ok(Json(warrior))
}

pub async fn search_warriors(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<Vec<Warrior>>, (StatusCode, String)> {
    let start = SystemTime::now();

    let query_key = format!("warriors:{:?}", params);

    // Try to fetch the result from Redis cache
    if let Ok(mut redis_conn) = state.redis_store.get().await {
        println!("Fetching warriors from cache {}", query_key);
        if let Ok(warriors_json) = redis_conn.get::<_, String>(&query_key).await.map_err(|err| {
            eprintln!("Failed to fetch warriors from cache: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }) {
            let warriors: Vec<Warrior> = serde_json::from_str(&warriors_json).unwrap();
            return Ok(Json(warriors));
        }        
    }


    // If not found in cache, execute the query and cache the result
    let query = r#"
        SELECT warriors.*,
            (SELECT array_agg(skills.name)
                FROM skills
                INNER JOIN warrior_skills ON skills.id = warrior_skills.skill_id
                WHERE warrior_skills.warrior_id = warriors.id
            ) AS fight_skills
        FROM warriors  LIMIT 50;"#;

    let warriors:Vec<Warrior>  = sqlx::query_as(query)
        .fetch_all(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;
    
    println!("Warriors fetched: {:?}", warriors);

    // Cache the result in Redis
    if let Ok(mut redis_conn) = state.redis_store.get().await {
        let warriors_json = serde_json::to_string(&warriors).unwrap();
        let _ = redis_conn.set::<_, String, ()>(&query_key, warriors_json).await.map_err(|err| {
            eprintln!("Failed to cache warriors: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
        println!("Warriors cached successfully");
    }
    match start.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("SystemTime taken to search warriors: {:?}", elapsed);
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    Ok(Json(warriors))
}

pub async fn count_warriors(
    State(state): State<AppState>,
) -> Result<Json<i64>, (StatusCode, String)>{
    let start = SystemTime::now();

    println!("Warriors counted");
    // TODO - Error handling

    let query = "SELECT COUNT(*) FROM warriors;";

    let row = sqlx::query(query)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;
    match start.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("SystemTime taken to count warrior: {:?}", elapsed);
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    Ok(Json(row.get::<i64, _>(0)))
    
}

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    println!("Error: {:?}", err);
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}