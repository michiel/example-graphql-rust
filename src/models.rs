use super::database_schema::users;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature")]
#[derive(Serialize, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature")]
pub struct GQLNewUser {
    pub id: String,
    pub name: String,
}
