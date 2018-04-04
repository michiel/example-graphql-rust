use juniper::FieldResult;
use juniper::RootNode;

use super::models::{User, NewUser};
use ::graphql_driver::GraphQLExecutor;
use ::database_queries::{db_create_user, db_find_user_by_id};

pub struct QueryRoot;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {
    field user(&executor, id: String) -> FieldResult<User> {
        let conn = executor.context().db_pool.get().unwrap();
        Ok(db_find_user_by_id(&conn, &id).unwrap())
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
