use async_graphql::*;
use serde::Serialize;
use surrealdb::opt::auth::Root;
use crate::{DbConnection};
use crate::database::SurrealDbActions;
use crate::types::expense::Expense;

#[derive(Default)]
pub struct ExpensesQuery;

#[derive(Serialize)]
struct Name {
    first: String,
    last: String
}
#[Object]
impl ExpensesQuery {
    async fn expense(&self,ctx: &Context<'_>,  provider: String, id: String) -> Option<Expense> {
        let db = ctx.data::<DbConnection>().ok()?;
        SurrealDbActions::login_as_user(db,"data" , "root", "root");

            .await.expect("failed to sign in as root");
        db.use_ns("data").use_db("person").await.expect("failed to use test namespace");
        db.set("name", Name {
            first: String::from("Tobie"),
            last: String::from("Morgan Hitchcock"),
        }).await.expect("failed to create person");
        // Use the variable in a subsequent query
        let data = db.query("CREATE person SET name = $name").await.expect("failed to run query");
        println!("{:?}", data);
        Some(Expense {
            id,
            date: "123131".to_string(),
            other_party: "31231".to_string(),
            amount: 10.0,
            reason: "12313".to_string(),
            saldo_after: 20.0,
        })
    }

    async fn expenses(&self, provider: String) -> Option<Vec<Expense>> {
        Some(Vec::new())
    }
}