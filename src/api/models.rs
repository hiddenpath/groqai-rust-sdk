use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::ModelList;

pub struct ModelsRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> ModelsRequestBuilder<'a> {
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    pub async fn list(self) -> Result<ModelList, GroqError> {
        let response = self.client.transport.get_json("models").await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}