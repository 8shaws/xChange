use chrono::NaiveDateTime;
use diesel::{prelude::*, query_builder::QueryId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, QueryId, Debug, Selectable, Deserialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hash_password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact_number: String,
    pub email_verified: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum LoginField {
//     Email(String),
//     ContactNumber(String),
// }
