use crate::database::SurrealDbActions;
use crate::types::auth::AuthParams;
use crate::{DbConnection, SurrealUserTokens};
use async_graphql::*;
use surrealdb::opt::auth::{Namespace, Root};

//TODO: errorhandling returning the error when db stuff fails
#[derive(Default)]
pub struct SignUpMutation;

#[Object]
impl SignUpMutation {
    async fn signup(&self, ctx: &Context<'_>, input: AuthParams) -> Option<bool> {
        let db = ctx.data::<DbConnection>().ok()?;
        let user_tokens = ctx.data::<SurrealUserTokens>().ok()?;

        let jwt = SurrealDbActions::signup_new_user(
            db,
            input.password,
            input.name,
            input.user_hash.clone(),
        )
        .await
        .expect("failed to create new user");

        user_tokens
            .lock()
            .expect("failed to lock user tokens")
            .insert(input.user_hash, jwt);

        Some(true)
    }
}
