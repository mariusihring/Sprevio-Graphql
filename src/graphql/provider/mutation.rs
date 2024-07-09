use async_graphql::*;

use crate::types::provider::{MapProviderInput, Provider};

#[derive(Default)]
pub struct ProviderMutation;


#[Object]
impl ProviderMutation {
    async fn create_provider(&self, id: String, file: Upload) -> Option<Provider> {
        Some(Provider{
            id,
            provider_name: String::from("Test")
        })
    }

    async fn map_statement_fields(&self, input: MapProviderInput) -> bool {
        true
    }


}