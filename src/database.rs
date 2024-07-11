use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::{Jwt, Namespace, Root};
use surrealdb::Surreal;

pub struct SurrealDbActions;

impl SurrealDbActions {
    pub async fn login_as_user(
        db: Surreal<Client>,
        namespace: &str,
        username: &str,
        password: &str,
    ) {
        db.signin(Namespace {
            namespace,
            username,
            password,
        })
        .await
        .expect("failed to sign into db");
    }

    pub async fn execute_root_action(
        db: Surreal<Client>,
        action: fn(),
    ) -> Result<bool, std::io::Error> {
        todo!()
    }

    pub async fn refresh_user_token(
        db: Surreal<Client>,
        jwt: String,
    ) -> Result<Jwt, std::io::Error> {
        todo!()
    }

    pub async fn signup_new_user(
        db: &Surreal<Client>,
        password: String,
        username: String,
        userhash: String,
    ) -> Result<Jwt, surrealdb::Error> {
        db.use_ns(userhash.clone())
            .await
            .expect("Couldnt switch namespace");
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .expect("failed to sign in as root user");
        db.query(format!(
            r#"DEFINE USER {} ON NAMESPACE PASSWORD "{}" ROLES OWNER"#,
            &username, &password
        ))
        .await
        .expect("couldnt create user");

        db.signin(Namespace {
            username: &username,
            password: &password,
            namespace: &userhash,
        })
        .await
    }
}
