use actix_web::{error, get, post, web, HttpResponse, Responder, Result};
use serde_json::json;

use crate::db::{self, DbPool};
use crate::models;

#[get("/login")]
async fn login() -> &'static str {
    "Login"
}

#[post("/register")]
async fn register(
    pool: web::Data<DbPool>,
    form: web::Json<models::RegisterUser>,
) -> Result<impl Responder> {
    let created_user = web::block(move || {
        let mut conn = pool.get()?;
        db::user_db_fn::insert_user(&mut conn, form.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(json!({
        "status": "ok",
        "user": created_user
    })))
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(login).service(register));
}
