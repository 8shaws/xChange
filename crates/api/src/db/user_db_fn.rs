use crate::models::User;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn _get_all_users(con: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let result = users.load::<User>(con).expect("Error loading users");
    println!("Displaying {} users", result.len());

    result
}
