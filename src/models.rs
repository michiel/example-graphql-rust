use super::database_schema::users;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature")]
#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub active: bool,
}

#[derive(Insertable)]
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
