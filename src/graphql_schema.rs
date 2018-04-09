use juniper::FieldResult;
use juniper::RootNode;

use super::models::*;
use ::graphql_driver::GraphQLExecutor;
use ::database_queries::*;

#[derive(GraphQLObject)]
#[graphql(description = "Connection")]
pub struct UserConnection {
    #[graphql(name="totalCount")]
    pub total_count: i32,
    #[graphql(description="This contains the User results")]
    pub edges: Vec<User>,
    #[graphql(name="pageInfo")]
    pub page_info: PageInfo,
    pub cursor: String,
}

#[derive(GraphQLInputObject)]
pub struct UsersFilterParams {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub active: Option<bool>,
}

impl Default for UsersFilterParams {
    fn default() -> Self {
        UsersFilterParams {
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
                filter: Option<UsersFilterParams>,
                paging: Option<PagingParams>
               ) -> FieldResult<UserConnection> {

        let conn = executor.context().db_pool.get()?;
        let filter = filter.unwrap_or(UsersFilterParams::default());
        let paging = paging.unwrap_or(PagingParams::default());

        let res = db_find_users(&conn, &paging)?;

        Ok(
            UserConnection {
                total_count: res.count,
                edges: res.items,
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
