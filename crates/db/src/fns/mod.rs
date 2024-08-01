use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2};

use std::env;

pub mod user_db_fn;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn _establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error Connecting to {}", database_url))
}

pub fn initialize_db_pool() -> DbPool {
    let con_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(con_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
