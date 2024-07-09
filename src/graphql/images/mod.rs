use async_graphql::*;


#[derive(Default)]
pub struct ImageMutation;


#[Object]
impl ImageMutation {
    async fn upload_image(&self, file: Upload) -> Option<String> {
        Some(String::from("uuid"))
    }
}