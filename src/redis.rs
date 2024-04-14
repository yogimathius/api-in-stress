use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

use bb8_redis::bb8;

pub async fn create_pool() -> bb8::Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new("redis://redis:6379").unwrap();
    println!("created redis connection manager {:?}", manager);
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    {
        // ping the database before starting
        let mut conn = pool.get().await.unwrap();
        conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
        let result: String = conn.get("foo").await.unwrap();
        assert_eq!(result, "bar");
    }
    tracing::debug!("successfully connected to redis and pinged it");
    pool
}