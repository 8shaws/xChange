use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub fn initialize_redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
