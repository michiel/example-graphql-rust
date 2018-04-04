use juniper::FieldResult;
use juniper::RootNode;

use super::models::{User, NewUser, GQLNewUser};
use super::database_driver::CreateUser;
use ::graphql_driver::GraphQLExecutor;

pub struct QueryRoot;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {
    field user(&executor, id: String) -> FieldResult<User> {
        Ok(User{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: GraphQLExecutor |&self| {
    field createUser(&executor, new_user: GQLNewUser) -> FieldResult<User> {
        let fut = executor.context().db_addr.send(CreateUser{
            name: new_user.name
        });
        Ok(User{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
