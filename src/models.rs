use super::database_schema::users;

pub struct DBQueryResult<T> {
    pub items: Vec<T>,
    pub count: i32,
}

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
#[graphql(description = "A humanoid creature")]
#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub active: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct DbNewUser<'a> {
    pub uuid: &'a str,
    pub name: &'a str,
    pub active: bool,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature")]
pub struct NewUser {
    pub name: String,
    pub active: bool,
}
