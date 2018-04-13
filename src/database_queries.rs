use uuid::Uuid;
use diesel;
use diesel::prelude::*;
use super::models::*;
use chrono::NaiveDateTime;

fn generate_uuid() -> String {
    let uuid: String = format!("{}", Uuid::new_v4());
    uuid
}

pub fn db_create_user(conn: &SqliteConnection, new_user: &NewUser) -> Result<User, String> {
    use database_schema::users::dsl;

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

pub fn db_update_user(conn: &SqliteConnection, uuid: &str, user: &NewUser) -> Result<User, String> {
    use database_schema::users::dsl;
    let res = dsl::users.filter(dsl::uuid.eq(&uuid));
    diesel::update(res).set(user).execute(&*conn);
    db_find_user_by_uuid(&conn, &uuid)
}

pub fn db_find_user_by_uuid(conn: &SqliteConnection, uuid: &str) -> Result<User, String> {
    use database_schema::users::dsl;

    let mut items = dsl::users
        .filter(dsl::uuid.eq(&uuid))
        .load::<User>(&*conn)
        .expect("Error loading user");

    Ok(items.pop().unwrap())
}

pub fn db_find_users(
    conn: &SqliteConnection,
    filter: &UsersFilterParams,
    paging: &PagingParams,
) -> Result<DBQueryResult<User>, String> {
    use database_schema::users::dsl;

    let limit = paging.get_limit() as i64;
    let current_cursor = NaiveDateTime::from_timestamp(paging.get_cursor(), 0);

    let mut query = dsl::users.into_boxed().order(dsl::created_at);

    if let Some(ref res) = filter.active {
        query = query.filter(dsl::active.eq(res));
    }

    if let Some(ref res) = filter.uuid {
        query = query.filter(dsl::uuid.eq(res));
    }

    let items = query
        .filter(dsl::created_at.gt(current_cursor))
        .limit(limit)
        .load::<User>(&*conn)
        .expect("Error loading users");

    let next_cursor = match items.last() {
        Some(item) => Some(format!("{}", item.created_at.timestamp())),
        None => None,
    };

    let has_more = (items.len() as i64) == limit;

    Ok(DBQueryResult {
        items: items,
        cursor: next_cursor,
        has_more: has_more,
    })
}
