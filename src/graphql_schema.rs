use juniper::FieldResult;
use juniper::RootNode;

use super::models::{User, NewUser};
use ::graphql_driver::GraphQLExecutor;
use ::database_queries::{db_create_user, db_find_user_by_uuid, db_find_users};

#[derive(GraphQLObject)]
#[graphql(description = "Page info")]
pub struct PageInfo {
    pub startCursor: String,
    pub endCursor: String,
    pub hasNextPage: bool,
}

#[derive(GraphQLObject)]
#[graphql(description = "Connection")]
pub struct UserConnection {
    pub totalCount: Option<i32>,
    pub edges: Vec<User>,
    pub pageInfo: PageInfo,
    pub cursor: String,
}

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
        Ok(db_find_users(&conn, limit).unwrap())
    }

    field usersConnection(&executor) -> FieldResult<UserConnection> {
        let conn = executor.context().db_pool.get().unwrap();
        Ok(
            UserConnection {
                totalCount: Some(5),
                edges: db_find_users(&conn, 5).unwrap(),
                pageInfo: PageInfo {
                    startCursor: "123".to_owned(),
                    endCursor: "123".to_owned(),
                    hasNextPage: true,
                },
                cursor: "123".to_owned(),
            }
        )
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
