use serde_json;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use sqlx::{postgres::PgPool, Execute, Postgres, QueryBuilder, Row};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, Query, State},
    http::{request::Parts, StatusCode},
    Json,
};
use redis::AsyncCommands; // Import the AsyncCommands trait to use the `set` method

use crate::models::{Warrior, NewWarrior};
use std::collections::HashMap;
use tower::BoxError;

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);
type RedisPool = Pool<RedisConnectionManager>;
pub struct RedisPoolWrapper(pub RedisPool);

#[derive(Clone)]
pub struct AppState {
    pub(crate) db_store: sqlx::PgPool,
    pub(crate) redis_store:  Pool<RedisConnectionManager>,
}

#[async_trait]
impl<S> FromRequestParts<S> for RedisPoolWrapper
where
    RedisPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let redis_pool = RedisPool::from_ref(state);
        Ok(Self(redis_pool))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub async fn create_warrior(
    State(state): State<AppState>,
    Json(warrior): Json<NewWarrior>
) ->  Result<Json<Warrior>, (StatusCode, String)>{
    println!("Creating warrior: {:?}", warrior);

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"INSERT INTO warriors (name, dob) VALUES ('"#
    );

    query_builder.push(warrior.name);
    query_builder.push(r#"', '"#);
    query_builder.push(warrior.dob);
    query_builder.push(r#"') RETURNING id, name, dob;"#);
    // TODO - Error handling

    let row = sqlx::query(query_builder.build().sql())
    .fetch_one(&state.db_store)
    .await
    .map_err(|err| internal_error(err))?;

    let warrior = Warrior {
        id: row.get::<i32, _>("id").to_string(),
        name: row.get::<String, _>("name"),
        dob: row.get::<String, _>("dob"),
    };

    Ok(Json(warrior))
}

pub async fn get_warrior(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Warrior>, (StatusCode, String)> {
    println!("Warrior fetched for id: {:?}", user_id);

    let query = format!(
        r#"SELECT id, name, dob
        FROM warriors
        WHERE id = {};"#,
        user_id
    );

    let row = sqlx::query(&query)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;

    let warrior = Warrior {
        id: row.get::<i32, _>("id").to_string(),
        name: row.get::<String, _>("name"),
        dob: row.get::<String, _>("dob"),
    };

    Ok(Json(warrior))
}

pub async fn search_warriors(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<Vec<Warrior>>, (StatusCode, String)> {
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
    let query = "SELECT id, name, dob FROM warriors LIMIT 50;";

    let rows = sqlx::query(query)
        .fetch_all(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;

    let warriors = rows
        .into_iter()
        .map(|row| Warrior {
            id: row.get::<i32, _>("id").to_string(),
            name: row.get::<String, _>("name"),
            dob: row.get::<String, _>("dob"),
        })
        .collect();

    // Cache the result in Redis
    if let Ok(mut redis_conn) = state.redis_store.get().await {
        let warriors_json = serde_json::to_string(&warriors).unwrap();
        let _ = redis_conn.set::<_, String, ()>(&query_key, warriors_json).await.map_err(|err| {
            eprintln!("Failed to cache warriors: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    }

    Ok(Json(warriors))
}

pub async fn count_warriors(
    State(state): State<AppState>,
) -> Result<Json<i64>, (StatusCode, String)>{
    println!("Warriors counted");
    // TODO - Error handling

    let query = "SELECT COUNT(*) FROM warriors;";

    let row = sqlx::query(query)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))?;

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