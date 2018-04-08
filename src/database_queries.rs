use uuid::Uuid;
use diesel;
use diesel::Connection;
use models;
use diesel::prelude::*;
use super::models::{User, NewUser, DbNewUser};

fn generate_uuid() -> String {
    let uuid : String = format!("{}", Uuid::new_v4());
    uuid
}

pub fn db_create_user(conn: &SqliteConnection, new_user: &NewUser) -> Result<User, String> {
    use ::database_schema::users::dsl;

    let uuid = generate_uuid();

    let user = DbNewUser {
        uuid: &uuid,
        name: &new_user.name,
        active: true,
    };

    diesel::insert_into(dsl::users)
        .values(&user)
        .execute(&*conn)
        .expect("Error inserting user");

    db_find_user_by_uuid(&conn, &uuid)

}

pub fn db_find_user_by_uuid(conn: &SqliteConnection, uuid: &str) -> Result<User, String> {
    use ::database_schema::users::dsl;

    let mut items = dsl::users
        .filter(dsl::uuid.eq(&uuid))
        .load::<models::User>(&*conn)
        .expect("Error loading user");

    Ok(items.pop().unwrap())
}

pub fn db_find_users(conn: &SqliteConnection, limit:i64) -> Result<Vec<User>, String> {
    use ::database_schema::users::dsl::*;

    let items = users
        .limit(limit)
        .load::<models::User>(&*conn)
        .expect("Error loading users");

    Ok(items)
}




