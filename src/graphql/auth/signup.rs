use async_graphql::*;
use surrealdb::opt::auth::{Namespace, Root};
use crate::{DbConnection, SurrealUserTokens};
use crate::types::auth::AuthParams;


//TODO: errorhandling returning the error when db stuff fails
#[derive(Default)]
pub struct SignUpMutation;
#[Object]
impl SignUpMutation {
    async fn signup(&self,ctx: &Context<'_>,  input: AuthParams) -> Option<bool> {
        let db = ctx.data::<DbConnection>().ok()?;
        let user_tokens = ctx.data::<SurrealUserTokens>().ok()?;
        db.use_ns(input.user_hash.clone()).await.expect("Couldnt switch namespace");
        db.signin(Root{
            username: "root",
            password: "root"
        }).await.expect("failed to sign in as root user");
        db.query(format!(r#"DEFINE USER {} ON NAMESPACE PASSWORD "{}" ROLES OWNER"#, input.name, input.password)).await.expect("couldnt create user");
        let jwt = db.signin(Namespace{
            username: &input.name,
            password: &input.password,
            namespace: &input.user_hash
        }).await.expect("Failed to sign in as namespace user");

        let token = jwt.as_insecure_token();
        user_tokens.lock().expect("failed to lock user tokens").insert(input.user_hash, token.to_string());
        println!("{:?}", user_tokens);

        Some(true)
    }
}