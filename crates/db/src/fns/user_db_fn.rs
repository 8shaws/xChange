extern crate diesel;

use crate::models::{self, User};
use common::types;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::{self, Uuid};

use super::DbError;

// inserts new user to db based on RegisterUser struct
pub fn insert_user(
    con: &mut PgConnection,
    user: types::user::RegisterUser,
    hash: String,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: uuid::Uuid::new_v4(),
        username: user.username,
        hash_password: hash,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        email_verified: Some(false),
        contact_number: user.contact_number,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(users).values(&new_user).execute(con)?;
    Ok(new_user)
}

pub fn get_user_by_email(
    con: &mut PgConnection,
    form_email: &str,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user_result = users
        .filter(email.eq(form_email))
        .select(User::as_select())
        .get_result::<User>(con)
        .optional()?;

    Ok(user_result)
}

pub fn get_user_by_contact(
    con: &mut PgConnection,
    contact: &str,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user_result = users
        .filter(contact_number.eq(contact))
        .select(User::as_select())
        .get_result::<User>(con)
        .optional()?;

    Ok(user_result)
}

pub fn verify_user(con: &mut PgConnection, user_id: String) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;
    let user_id = Uuid::parse_str(&user_id).expect("Error parsing user id");

    diesel::update(users.filter(id.eq(user_id)))
        .set(email_verified.eq(true))
        .execute(con)
        .map_err(|e| DbError::from(e))
}

pub fn get_user_mail_by_id(
    con: &mut PgConnection,
    user_id: String,
) -> Result<Option<String>, DbError> {
    use crate::schema::users::dsl::*;

    let user_id = Uuid::parse_str(&user_id).expect("Error parsing user id");

    let mail = users
        .filter(id.eq(user_id))
        .select(email)
        .get_result::<String>(con)
        .optional()
        .map_err(|e| DbError::from(e))?;

    Ok(mail)
}

pub fn is_user_verified(con: &mut PgConnection, user_id: String) -> Result<bool, DbError> {
    use crate::schema::users::dsl::*;

    let user_id = Uuid::parse_str(&user_id).expect("Error parsing user id");

    let verified = users
        .filter(id.eq(user_id))
        .select(email_verified)
        .get_result::<Option<bool>>(con)
        .optional()
        .map_err(|e| DbError::from(e))?;

    match verified.unwrap() {
        Some(v) => Ok(v),
        None => Ok(false),
    }
}
