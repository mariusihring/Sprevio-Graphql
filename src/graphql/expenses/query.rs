use async_graphql::*;
use crate::types::expense::Expense;

#[derive(Default)]
pub struct ExpensesQuery;


#[Object]
impl ExpensesQuery {
    async fn expense(&self, provider: String, id: String) -> Option<Expense> {
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