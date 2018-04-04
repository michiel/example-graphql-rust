use uuid;
use diesel;
use diesel::Connection;
use models;
use diesel::prelude::*;
use super::models::{User, NewUser, DbNewUser};

fn generate_uuid() -> String {
    let uuid = format!("{}", uuid::Uuid::new_v4());
    uuid
}

pub fn db_create_user(conn: &SqliteConnection, new_user: &NewUser) -> Result<User, String> {
    let uuid = generate_uuid();
    let user = DbNewUser {
        id: &uuid,
        name: &new_user.name,
    };

    use ::database_schema::users::dsl::*;
    diesel::insert_into(users)
        .values(&user)
        .execute(&*conn)
        .expect("Error inserting user");

    let mut items = users
        .filter(id.eq(&uuid))
        .load::<models::User>(&*conn)
        .expect("Error loading user");

    Ok(items.pop().unwrap())
}

pub fn db_find_user_by_id(conn: &SqliteConnection, uuid: &str) -> Result<User, String> {
    use ::database_schema::users::dsl::*;

    let mut items = users
        .filter(id.eq(&uuid))
        .load::<models::User>(&*conn)
        .expect("Error loading user");

    Ok(items.pop().unwrap())
}




