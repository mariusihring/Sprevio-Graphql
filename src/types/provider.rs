use async_graphql::*;

#[derive(InputObject)]
pub struct MapProviderInput {
    pub provider_name: String,
    pub date: String,
    pub other_party: String,
    pub amount: String,
    pub reason: String,
    pub saldo_after: String,
}

#[derive(SimpleObject)]
pub struct Provider {
    pub id: String,
    pub provider_name: String,
}

#[derive(SimpleObject)]
pub struct CsvColumn {
    pub row: String,
    pub row_type: String,
}
