use actix_web::*;
use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2;
use std;
use dotenv;

use models;

pub fn get_db_connection_pool() -> DBPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Did not find DATABASE_URL in config");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create r2d2 pool")
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
        use database_queries::db_create_user;
        let conn: &SqliteConnection = &self.0.get().unwrap();
        let user = ::models::NewUser {
            name: msg.name,
            active: true,
        };
        Ok(db_create_user(&conn, &user).unwrap())
    }
}
