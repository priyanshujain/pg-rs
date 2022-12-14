use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use pg_rs::models::user::{NewUser, UserModel};
use pg_rs::schema::users::dsl::*;

/// Create a new `User` and add it to the database.
///
/// The user's email will be set to '<uuid>@example.com'.
pub fn make_user(conn: &PgConnection) -> UserModel {
    let new_email = format!(
        "{username}@example.com",
        username = Uuid::new_v4().to_hyphenated().to_string()
    );
    let new_password_hash = UserModel::make_password_hash("testtest");
    let new_user = NewUser {
        email: new_email,
        password_hash: new_password_hash,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<UserModel>(conn)
        .expect("Error saving new post")
}
