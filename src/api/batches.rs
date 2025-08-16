use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{Batch, BatchList};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct BatchCreateRequest {
    pub input_file_id: String,
    pub endpoint: String,
    pub completion_window: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

pub struct BatchRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> BatchRequestBuilder<'a> {
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    pub async fn create(self, req: BatchCreateRequest) -> Result<Batch, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_json("batches", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn retrieve(self, batch_id: String) -> Result<Batch, GroqError> {
        let path = format!("batches/{}", batch_id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn list(self, after: Option<String>, limit: Option<u32>) -> Result<BatchList, GroqError> {
        let mut params = Vec::new();
        if let Some(after_id) = after {
            params.push(("after", after_id));
        }
        if let Some(limit_val) = limit {
            params.push(("limit", limit_val.to_string()));
        }
        
        if params.is_empty() {
            let response = self.client.transport.get_json("batches").await?;
            serde_json::from_value(response).map_err(GroqError::from)
        } else {
            let response = self.client.transport.get_with_params("batches", &params).await?;
            serde_json::from_value(response).map_err(GroqError::from)
        }
    }

    pub async fn cancel(self, batch_id: String) -> Result<Batch, GroqError> {
        let path = format!("batches/{}/cancel", batch_id);
        let body = serde_json::Value::Null;
        let response = self.client.transport.post_json(&path, &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}