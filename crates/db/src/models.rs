use chrono::NaiveDateTime;
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::{prelude::*, query_builder::QueryId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::kyc;
use crate::schema::users;

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

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

#[derive(Queryable, Insertable, Serialize, QueryId, Debug, Selectable, Deserialize, Clone)]
#[diesel(table_name = kyc)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Kyc {
    pub id: Uuid,
    pub user_id: Uuid,
    pub document_type: String,
    pub document_number: String,
    pub issue_country: String,
    pub expiry_date: NaiveDateTime,
    pub document_front_url: String,
    pub document_back_url: String,
    pub selfie_url: String,
    pub verification_status: String,
    pub submitted_at: NaiveDateTime,
    pub verified_at: Option<NaiveDateTime>,
    pub rejected_at: Option<NaiveDateTime>,
    pub rejection_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
