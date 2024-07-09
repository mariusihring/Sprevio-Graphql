use async_graphql::*;
use crate::types::expense::Expense;

#[derive(Default)]
pub struct ProviderQuery;


#[Object]
impl ProviderQuery {
    async fn provider(&self, id: String) -> Option<Expense> {
        Some(Expense {
            id,
            date: "123131".to_string(),
            other_party: "31231".to_string(),
            amount: 10.0,
            reason: "12313".to_string(),
            saldo_after: 20.0,
        })
    }

    async fn providers(&self) -> Option<Vec<Expense>> {
        Some(Vec::new())
    }
}