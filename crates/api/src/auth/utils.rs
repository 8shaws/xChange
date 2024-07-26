use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::{encode, errors::Error, Algorithm, EncodingKey, Header};
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
