use chrono;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub fn initialize_redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn current_time() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
