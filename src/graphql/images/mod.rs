use async_graphql::*;

use crate::types::provider::{MapProviderInput, Provider};

#[derive(Default)]
pub struct ImageMutation;


#[Object]
impl ImageMutation {
    async fn upload_image(&self, file: Upload) -> bool {
        true
    }



}