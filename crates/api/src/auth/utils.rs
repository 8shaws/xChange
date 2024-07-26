use argon2::Argon2;
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::{encode, errors::Error, Algorithm, EncodingKey, Header};
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
    iat: usize,
}

pub fn generate_token(payload: &str) -> Result<String, Error> {
    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600;
    let claims = Claims {
        sub: payload.to_string(),
        company: "xChange".to_owned(),
        exp: expiration_time,
        iat: expiration_time - 3600,
    };
    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set!");

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )
}

pub fn verify_token(token: &str) -> Result<String, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set!");

    decode::<Claims>(token, &DecodingKey::from_secret(key.as_ref()), &validation)
        .map(|data| data.claims.sub)
}

pub fn verify_password(password: &str, hash_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash_password).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn hash_password(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    (password_hash, salt.to_string())
}
