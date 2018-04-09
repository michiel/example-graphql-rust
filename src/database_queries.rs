use uuid::Uuid;
use diesel;
use diesel::Connection;
use diesel::prelude::*;
use super::models::*;

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
        .load::<User>(&*conn)
        .expect("Error loading user");

    Ok(items.pop().unwrap())
}

pub fn db_find_users(conn: &SqliteConnection, paging: &PagingParams) -> Result<DBQueryResult<User>, String> {
    use ::database_schema::users::dsl::*;

    let base = users;
    let count = base
        .count()
        .get_result(&*conn);

    let items = base
        .limit(paging.get_limit() as i64)
        .load::<User>(&*conn)
        .expect("Error loading users");

    Ok(DBQueryResult {
        items: items,
        count: count.unwrap_or(0) as i32,
    })
}




