use actix_web::{error, post, web, HttpResponse, Responder, Result};
use serde_json::json;

use crate::auth;
use crate::auth::middleware::ExtractClientId;
use crate::db::{self};
use crate::models::*;

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
