use actix_web::{error, get, post, web, HttpResponse, Responder, Result};
use serde_json::json;

use crate::auth;
use crate::auth::middleware::ExtractClientId;
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

    let token = match auth::utils::generate_token(&created_user.id.to_string()) {
        Ok(t) => t,
        Err(_) => return Err(error::ErrorInternalServerError("Error generating token")),
    };

    Ok(HttpResponse::Created().json(json!({
        "status": "ok",
        "user": created_user,
        "jwt": token
    })))
}

async fn get_orders() -> impl Responder {
    HttpResponse::Ok().body("Orders")
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(login)
            .service(register)
            .service(
                web::resource("/orders")
                    .wrap(ExtractClientId)
                    .route(web::get().to(get_orders)),
            ),
    );
}
