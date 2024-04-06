use diesel::prelude::*;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, Query, State},
    http::{request::Parts, StatusCode},
    Json
};

use crate::{models::{Warrior, NewWarrior}, schema::warriors};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use std::collections::HashMap;
use tower::BoxError;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn create_warrior(
    State(pool): State<Pool>,
    Json(new_warrior): Json<NewWarrior>
) ->  Result<Json<Warrior>, (StatusCode, String)>{
    println!("Creating warrior: {:?}", new_warrior);
    // TODO - Error handling
    let mut conn = pool.get().await.map_err(internal_error).unwrap();

    let res = diesel::insert_into(warriors::table)
        .values(new_warrior)
        .returning(Warrior::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub struct DatabaseConnection(
    pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub async fn get_warrior(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<u32>) -> Result<Json<Warrior>, (StatusCode, String)> {
    println!("Warrior fetched for id: {:?}", user_id);

    // TODO - Error handling
    let res = warriors::table
        .filter(warriors::id.eq(user_id as i32)) // Assuming the primary key column is `id`
        .first(&mut conn)
        .await
        .map_err(internal_error)
        .unwrap();

    Ok(Json(res))
}

pub async fn search_warriors(
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<Vec<Warrior>>, (StatusCode, String)> {
    println!("Searching warriors with params: {:?}", params);
    let res = warriors::table
        .select(Warrior::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn count_warriors(DatabaseConnection(mut conn): DatabaseConnection,) -> Result<Json<i64>, (StatusCode, String)>{
    println!("Warriors counted");
    // TODO - Error handling
    let res = warriors::table
        .count()
        .get_result(&mut conn)
        .await
        .map_err(internal_error)
        .unwrap();
    Ok(Json(res))
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