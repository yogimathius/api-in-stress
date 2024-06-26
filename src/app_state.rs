use crate::{database::Database, redis::RedisDatabase, utilities::internal_error};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sqlx::postgres::PgPool;
pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);
type RedisPool = Pool<RedisConnectionManager>;
pub struct RedisPoolWrapper(pub RedisPool);

#[derive(Clone)]
pub struct AppState {
    pub(crate) database: Database,
    pub(crate) database_shard: String,
    pub(crate) redis_store: RedisDatabase,
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
