use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenvy::dotenv;
use std::env;

pub mod user_db_fn;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error Connecting to {}", database_url))
}
