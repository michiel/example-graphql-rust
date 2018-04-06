use juniper::FieldResult;
use juniper::RootNode;

use super::models::{User, NewUser};
use ::graphql_driver::GraphQLExecutor;
use ::database_queries::{db_create_user, db_find_user_by_uuid, db_find_users};

#[derive(GraphQLObject)]
#[graphql(description = "Page info")]
pub struct PageInfo {
    #[graphql(name="startCursor")]
    pub start_cursor: String,
    #[graphql(name="endCursor")]
    pub end_cursor: String,
    #[graphql(name="hasNextPage")]
    pub has_next_page: bool,
}

const DEFAULT_PAGE_SIZE :i32 = 20;

#[derive(GraphQLInputObject)]
pub struct PagingParams {
    pub limit: Option<i32>,
    pub cursor: Option<String>
}

impl PagingParams {
    pub fn get_limit(&self) -> i32 {
        match self.limit {
            None => DEFAULT_PAGE_SIZE,
            Some(limit) => limit
        }
    }
}

impl Default for PagingParams {
    fn default() -> Self {
        PagingParams {
            limit: Some(DEFAULT_PAGE_SIZE),
            cursor: None,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "Connection")]
pub struct UserConnection {
    #[graphql(name="totalCount")]
    pub total_count: Option<i32>,
    pub edges: Vec<User>,
    #[graphql(name="pageInfo")]
    pub page_info: PageInfo,
    pub cursor: String,
}

#[derive(GraphQLInputObject)]
pub struct UsersQueryParams {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub active: Option<bool>,
}

impl Default for UsersQueryParams {
    fn default() -> Self {
        UsersQueryParams {
            uuid: None,
            name: None,
            active: None,
        }
    }
}

pub struct QueryRoot;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {
    field user(&executor, uuid: String) -> FieldResult<User> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_find_user_by_uuid(&conn, &uuid)?)
    }

    field users(&executor,
                limit: Option<i32> as "Optional limit, default 20"
                ) -> FieldResult<Vec<User>> {
        let conn = executor.context().db_pool.get()?;
        let limit = limit.unwrap_or(20) as i64;
        Ok(db_find_users(&conn, limit)?)
    }

    field usersConnection(&executor,
                          params: Option<UsersQueryParams>,
                          paging: Option<PagingParams>
                          ) -> FieldResult<UserConnection> {

        let conn = executor.context().db_pool.get()?;
        let params = params.unwrap_or(UsersQueryParams::default());
        let paging = paging.unwrap_or(PagingParams::default());
        let res = db_find_users(&conn, (paging.get_limit() as i64))?;

        Ok(
            UserConnection {
                total_count: Some(15),
                edges: res,
                page_info: PageInfo {
                    start_cursor: "123".to_owned(),
                    end_cursor: "123".to_owned(),
                    has_next_page: true,
                },
                cursor: "123".to_owned(),
            }
        )
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: GraphQLExecutor |&self| {
    field createUser(&executor, user: NewUser) -> FieldResult<User> {
        let conn = executor.context().db_pool.get()?;
        Ok(db_create_user(&conn, &user)?)
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
