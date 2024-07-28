extern crate diesel;

use crate::auth;
use crate::models::{self, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid;

use super::DbError;

// inserts new user to db based on RegisterUser struct
pub fn insert_user(
    con: &mut PgConnection,
    user: models::RegisterUser,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let hash = auth::utils::hash_password(&user.password);

    let new_user = models::User {
        id: uuid::Uuid::new_v4(),
        username: user.username,
        hash_password: hash.0,
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
