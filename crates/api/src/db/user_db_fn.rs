use crate::models::{self, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid;

use super::DbError;

pub fn _get_all_users(con: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let result = users.load::<User>(con).expect("Error loading users");
    println!("Displaying {} users", result.len());

    result
}

// inserts new user to db based on RegisterUser struct
pub fn insert_user(
    con: &mut PgConnection,
    user: models::RegisterUser,
) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: uuid::Uuid::new_v4(),
        username: user.username,
        password: user.password,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        contact_number: user.contact_number,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(con)
        .map_err(Into::into)
}
