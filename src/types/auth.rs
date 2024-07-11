use async_graphql::InputObject;

#[derive(InputObject)]
pub struct AuthParams {
    pub name: String,
    pub password: String,
    pub user_hash: String,
}
