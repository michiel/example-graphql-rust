use juniper::FieldResult;
use juniper::RootNode;

use super::models::{User, NewUser};
use ::graphql_driver::GraphQLExecutor;
use ::database_queries::{db_create_user, db_find_user_by_uuid, db_find_users};

pub struct QueryRoot;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {
    field user(&executor, uuid: String) -> FieldResult<User> {
        let conn = executor.context().db_pool.get().unwrap();
        Ok(db_find_user_by_uuid(&conn, &uuid).unwrap())
    }

    field users(&executor,
                limit: Option<i32> as "Optional limit, default 20"
                ) -> FieldResult<Vec<User>> {
        let conn = executor.context().db_pool.get().unwrap();
        let limit = limit.unwrap_or(20) as i64;
        Ok(db_find_users( &conn, limit).unwrap())
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: GraphQLExecutor |&self| {
    field createUser(&executor, user: NewUser) -> FieldResult<User> {
        let conn = executor.context().db_pool.get().unwrap();
        Ok(db_create_user(&conn, &user).unwrap())
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
