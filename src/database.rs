use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::{Root, Namespace};
use surrealdb::Surreal;

pub struct SurrealDbActions;


impl SurrealDbActions {
    pub async fn login_as_user(db: Surreal<Client>, namespace: &str, username: &str, password: &str) {
        db.signin(Namespace {
            namespace,
            username,
            password,
        }).await.expect("failed to sign into db");
    }
}