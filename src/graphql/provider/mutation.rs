use async_graphql::*;

use crate::types::provider::{CsvColumn, MapProviderInput, Provider};

#[derive(Default)]
pub struct ProviderMutation;


#[Object]
impl ProviderMutation {
    async fn create_provider(&self, id: String, file: Upload) -> Vec<CsvColumn> {
        Vec::new()
    }

    async fn map_statement_fields(&self, input: MapProviderInput) -> bool {
        true
    }
}