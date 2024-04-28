use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use hyper::StatusCode;
use redis::AsyncCommands;

use bb8_redis::bb8;

#[derive(Clone)]
pub struct RedisDatabase {
    redis_store: Pool<RedisConnectionManager>,
}

impl RedisDatabase {
    pub async fn new() -> Self {
        let manager = RedisConnectionManager::new("redis://redis:6379").unwrap();
        println!("created redis connection manager from impl {:?}", manager);
        let pool = bb8::Pool::builder().build(manager).await.unwrap();

        {
            let mut conn = pool.get().await.unwrap();
            conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
            let result: String = conn.get("foo").await.unwrap();
            assert_eq!(result, "bar");
        }
        tracing::debug!("successfully connected to redis and pinged it");

        RedisDatabase { redis_store: pool }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let mut conn: bb8::PooledConnection<'_, bb8_redis::RedisConnectionManager> =
            self.redis_store.get().await.unwrap();
        match conn.get::<_, String>(key).await {
            Ok(value) => Some(value),
            Err(_err) => None,
        }
    }

    pub async fn set(&self, key: &str, value: String) {
        let mut conn: bb8::PooledConnection<'_, bb8_redis::RedisConnectionManager> =
            self.redis_store.get().await.unwrap();
        let _ = conn
            .set::<_, String, ()>(key, value)
            .await
            .map_err(|err: redis::RedisError| {
                eprintln!("Failed to cache warrior: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            });
    }
}
