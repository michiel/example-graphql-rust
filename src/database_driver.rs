//! Db executor actor
use uuid;
use diesel;
use actix_web::*;
use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use r2d2;
use std;
use dotenv;

use models;
use database_schema;

pub fn get_db_connection_pool() -> DBPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Did not find DATABASE_URL in config");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create r2d2 pool");
    pool
}

pub fn get_db_address() -> actix::Addr<Syn, DbExecutor> {
    SyncArbiter::start(3, move || DbExecutor(get_db_connection_pool()))
}

pub type DBPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct DbExecutor(DBPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// # Messages
/// ## CreateUser
///
pub struct CreateUser {
    pub name: String,
}

impl Message for CreateUser {
    type Result = Result<models::User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        use self::database_schema::users::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = models::NewUser {
            id: &uuid,
            name: &msg.name,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .expect("Error inserting person");

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<models::User>(conn)
            .expect("Error loading person");

        Ok(items.pop().unwrap())
    }
}

pub struct GetUser {
    pub name: String,
}

impl Message for GetUser {
    type Result = Result<models::User, Error>;
}

impl Handler<GetUser> for DbExecutor {
    type Result = Result<models::User, Error>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        use self::database_schema::users::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = models::NewUser {
            id: &uuid,
            name: &msg.name,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
            .expect("Error inserting person");

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<models::User>(conn)
            .expect("Error loading person");

        Ok(items.pop().unwrap())
    }
}
