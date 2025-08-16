use crate::client::GroqClient;
use crate::error::GroqError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct FineTuningCreateRequest {
    pub base_model: String,
    pub input_file_id: String,
    pub name: String,
    pub type_: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FineTuning {
    pub id: String,
    pub name: String,
    pub base_model: String,
    pub type_: String,
    pub input_file_id: String,
    pub created_at: u64,
    pub fine_tuned_model: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FineTuningList {
    pub object: String,
    pub data: Vec<FineTuning>,
}

pub struct FineTuningRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> FineTuningRequestBuilder<'a> {
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    pub async fn create(self, req: FineTuningCreateRequest) -> Result<FineTuning, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_json("v1/fine_tunings", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn list(self) -> Result<FineTuningList, GroqError> {
        let response = self.client.transport.get_json("v1/fine_tunings").await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn retrieve(self, id: String) -> Result<FineTuning, GroqError> {
        let path = format!("v1/fine_tunings/{}", id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn delete(self, id: String) -> Result<serde_json::Value, GroqError> {
        let path = format!("v1/fine_tunings/{}", id);
        self.client.transport.delete_json(&path).await
    }
}
