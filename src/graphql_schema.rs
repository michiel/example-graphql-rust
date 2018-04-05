use juniper::FieldResult;
use juniper::RootNode;

use super::models::{User, NewUser, GQLNewUser};
use ::graphql_driver::GraphQLExecutor;
use uuid;
use diesel;
use models;
use diesel::prelude::*;

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
    field createUser(&executor, gql_new_user: GQLNewUser) -> FieldResult<User> {
        use ::database_schema::users::dsl::*;
        let conn = executor.context().db_pool.get().unwrap();

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = models::NewUser {
            id: &uuid,
            name: &gql_new_user.name,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&conn)
            .expect("Error inserting person");

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<models::User>(&conn)
            .expect("Error loading person");

        Ok(items.pop().unwrap())

    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
