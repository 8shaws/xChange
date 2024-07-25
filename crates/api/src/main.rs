use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Builder;
use lazy_static::lazy_static;
use num_cpus;
use serde_json::json;
use std::time::SystemTime;

mod db;
mod route;
mod schema;

use crate::db::establish_connection;
use crate::route::user::user_config;

lazy_static! {
    static ref START_TIME: SystemTime = SystemTime::now();
}

async fn root() -> impl Responder {
    let uptime = match SystemTime::now().duration_since(*START_TIME) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    };
    HttpResponse::Ok().json(json!({
        "uptime": uptime,
        "status": "Api server is healthy!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let thread_count = num_cpus::get() - 6;

    Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let _connection = &mut establish_connection();

    println!("{:?}: Api Server is running on port: {}", *START_TIME, 8080);

    HttpServer::new(|| {
        App::new()
            .configure(user_config)
            .route("/", web::get().to(root))
            .route("/_health", web::get().to(root))
    })
    .workers(thread_count)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
