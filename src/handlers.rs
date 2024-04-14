use sqlx::{postgres::PgPool, Execute, Postgres, QueryBuilder, Row};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, Query},
    http::{request::Parts, StatusCode},
    Json
};

use crate::models::{Warrior, NewWarrior};
use std::collections::HashMap;
use tower::BoxError;

pub async fn create_warrior(
    DatabaseConnection(mut conn): DatabaseConnection,
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
    .fetch_one(&mut conn)
    .await
    .map_err(|err| internal_error(err))?;

    let warrior = Warrior {
        id: row.get::<i32, _>("id").to_string(),
        name: row.get::<String, _>("name"),
        dob: row.get::<String, _>("dob"),
    };

    Ok(Json(warrior))
}

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

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

pub async fn get_warrior(
    DatabaseConnection(mut conn): DatabaseConnection,
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
        .fetch_one(&mut conn)
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
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<Vec<Warrior>>, (StatusCode, String)> {
    println!("Searching warriors with params: {:?}", params);
    
    let query = "SELECT id, name, dob FROM warriors LIMIT 50;";

    let rows = sqlx::query(query)
        .fetch_all(&mut conn)
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

    Ok(Json(warriors))
}

pub async fn count_warriors(DatabaseConnection(mut conn): DatabaseConnection,) -> Result<Json<i64>, (StatusCode, String)>{
    println!("Warriors counted");
    // TODO - Error handling

    let query = "SELECT COUNT(*) FROM warriors;";

    let row = sqlx::query(query)
        .fetch_one(&mut conn)
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