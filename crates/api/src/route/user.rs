use crate::auth;
use crate::middlewares::un_verify_user::UnVerifyUser;
use actix_web::{error, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use common::types::user::{LoginUser, RegisterUser, ResendOtpBody, VerifyEmailBody};
use r2d2_redis::redis;
use serde_json::json;

use crate::middlewares::{
    extract_client_id::{ExtractClientId, IdKey},
    rate_limit::RateLimiter,
    verify_user::VerifyUser,
};

use db::fns::user_db_fn;

use crate::types::AppState;

#[post("/login")]
async fn login(state: web::Data<AppState>, form: web::Json<LoginUser>) -> Result<impl Responder> {
    let form = form.into_inner();
    let user = web::block(move || {
        let mut con = state.db_pool.get()?;

        let db_user = if form.login_field.contains("@") {
            user_db_fn::get_user_by_email(&mut con, &form.login_field)
        } else {
            user_db_fn::get_user_by_contact(&mut con, &form.login_field)
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
                    "user": user,
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
        let hash = auth::utils::hash_password(&form.password);
        user_db_fn::insert_user(&mut conn, form.into_inner(), hash.0)
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

async fn verify_email(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<VerifyEmailBody>,
) -> Result<HttpResponse> {
    let id = req.extensions().get::<IdKey>().cloned();

    if let Some(id) = id {
        let redis_pool = state.redis_pool.clone();
        let db_pool = state.db_pool.clone();
        let id_clone = id.clone().0;
        let form_otp = form.otp.clone();

        let otp_result = web::block(move || {
            let mut redis_conn = redis_pool.get().map_err(|e| e.to_string())?;
            let mut db_conn = db_pool.get().map_err(|e| e.to_string())?;

            let mail = user_db_fn::get_user_mail_by_id(&mut db_conn, id.0);

            match mail {
                Ok(Some(mail)) => {
                    let otp: Option<String> = redis::cmd("GET")
                        .arg(format!("otp:{}", mail))
                        .query(&mut *redis_conn)
                        .map_err(|e| e.to_string())?;

                    match otp {
                        Some(otp) => Ok(Some(otp)),
                        None => Ok(None),
                    }
                }
                Ok(None) => return Ok(None),
                Err(e) => Err(e.to_string()),
            }
        })
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        let db_pool = state.db_pool.clone();

        match otp_result {
            Ok(Some(otp)) if otp == form_otp => {
                let _ = web::block(move || {
                    let mut db_conn = db_pool.get()?;
                    user_db_fn::verify_user(&mut db_conn, id_clone)
                })
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                Ok(HttpResponse::Ok().json(json!({
                    "status": "ok",
                    "message": "Email Verified!"
                })))
            }
            Ok(Some(_)) => Ok(HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Invalid OTP"
            }))),
            Ok(None) => Ok(HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "OTP not found, resend email?"
            }))),
            Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Internal Server Error"
            }))),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid token"
        })))
    }
}

pub async fn resend_otp(
    state: web::Data<AppState>,
    form: web::Json<ResendOtpBody>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let id_key = req.extensions().get::<IdKey>().cloned();

    match id_key {
        Some(id) => {
            let redis_pool = state.redis_pool.clone();
            let mail = form.mail.clone();

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

            if let Ok(_) = result {
                Ok(HttpResponse::Ok().json(json!({
                    "status": "ok",
                    "message": "OTP sent"
                })))
            } else {
                Ok(HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Internal Server Error"
                })))
            }
        }
        None => Ok(HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid token"
        }))),
    }
}

pub fn user_config(
    cfg: &mut web::ServiceConfig,
    app_state: web::Data<AppState>,
    rate_limiter: web::Data<RateLimiter>,
) {
    let verify_user = VerifyUser::new(app_state.clone());
    let un_verify_user = UnVerifyUser::new(app_state.clone());

    cfg.service(
        web::scope("/user")
            .service(login)
            .service(register)
            .service(
                web::resource("/orders")
                    .wrap(verify_user)
                    .wrap(ExtractClientId)
                    .route(web::get().to(get_orders)),
            )
            .service(
                web::resource("/verify_email")
                    .wrap(ExtractClientId)
                    .route(web::post().to(verify_email)),
            )
            .service(
                web::resource("/resend_otp")
                    .wrap(un_verify_user)
                    .wrap(ExtractClientId)
                    .route(web::post().to(resend_otp)),
            ),
    );
}
