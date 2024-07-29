use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmailVerifyData {
    pub id: String,
    pub mail: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyEmailBody {
    pub otp: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyUser {
    pub email_verified: Option<bool>,
}
