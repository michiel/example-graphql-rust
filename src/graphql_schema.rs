use juniper::FieldResult;
use juniper::RootNode;

use super::models::{User, GQLNewUser};

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field user(&executor, id: String) -> FieldResult<User> {
        Ok(User{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createUser(&executor, new_user: GQLNewUser) -> FieldResult<User> {
        Ok(User{
            id: "1234".to_owned(),
            name: new_user.name,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
