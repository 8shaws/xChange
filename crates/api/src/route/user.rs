use crate::auth;
use crate::auth::middleware::ExtractClientId;
use crate::db::{self};
use crate::models::*;
use actix_web::{error, post, web, HttpResponse, Responder, Result};
use r2d2_redis::redis;
use serde_json::json;

#[post("/login")]
async fn login(state: web::Data<AppState>, form: web::Json<LoginUser>) -> Result<impl Responder> {
    let form = form.into_inner();
    let user = web::block(move || {
        let mut con = state.db_pool.get()?;

        let db_user = if form.login_field.contains("@") {
            db::user_db_fn::get_user_by_email(&mut con, &form.login_field)
        } else {
            db::user_db_fn::get_user_by_contact(&mut con, &form.login_field)
        };
        db_user
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        Some(user) => {
            // Check if password is correct
            if auth::utils::verify_password(&form.password, &user.hash_password) {
                let token = match auth::utils::generate_token(&user.id.to_string()) {
                    Ok(t) => t,
                    Err(_) => {
                        return Err(error::ErrorInternalServerError("Error generating token"))
                    }
                };

                HttpResponse::Ok().json(json!({
                    "status": "ok",
                    "jwt": token,
                    "message": "Login successful"
                }))
            } else {
                HttpResponse::Unauthorized().json(json!({
                    "status": "error",
                    "message": "Invalid password"
                }))
            }
        }

        // User was not found; return 404 response with error message
        None => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "User with field not found"
        })),
    })
}

#[post("/register")]
async fn register(
    state: web::Data<AppState>,
    form: web::Json<RegisterUser>,
) -> Result<impl Responder> {
    let redis_pool = state.redis_pool.clone();

    let created_user = web::block(move || {
        let mut conn = state.db_pool.get()?;
        db::user_db_fn::insert_user(&mut conn, form.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let token = match auth::utils::generate_token(&created_user.id.to_string()) {
        Ok(t) => t,
        Err(_) => return Err(error::ErrorInternalServerError("Error generating token")),
    };

    let mail = created_user.email.clone();
    let result = web::block(move || {
        let mut conn = redis_pool.get().map_err(|e| e.to_string())?;
        let _: () = redis::cmd("LPUSH")
            .arg("user_email_verify")
            .arg(mail)
            .query(&mut *conn)
            .map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match result {
        Ok(_) => Ok(HttpResponse::Created().json(json!({
            "status": "ok",
            "user": created_user,
            "jwt": token,
            "mail_status": "ok"
        }))),
        Err(err) => Ok(HttpResponse::Created().json(json!({
            "status": "ok",
            "user": created_user,
            "jwt": token,
            "mail_status": err
        }))),
    }
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
