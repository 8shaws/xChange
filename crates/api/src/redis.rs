use deadpool_redis::{Config, Pool as RedisPool, Runtime};
use std::env;

pub fn initialize_redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1));
    pool.expect("Failed to create Redis Pool")
}
