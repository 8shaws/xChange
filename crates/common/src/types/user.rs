use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub login_field: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub contact_number: String,
}

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

#[derive(Serialize, Deserialize)]
pub struct ResendOtpBody {
    pub mail: String,
}
