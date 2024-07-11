use async_graphql::*;

#[derive(SimpleObject)]
pub struct Expense {
    pub id: String,
    pub date: String,
    pub other_party: String,
    pub amount: f64,
    pub reason: String,
    pub saldo_after: f64,
}
