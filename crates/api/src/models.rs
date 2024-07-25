use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact_number: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
