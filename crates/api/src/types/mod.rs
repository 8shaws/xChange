use serde::{Deserialize, Serialize};

use crate::redis::RedisPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: db::fns::DbPool,
    pub redis_pool: RedisPool,
}
