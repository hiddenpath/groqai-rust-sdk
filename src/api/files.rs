use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{File, FileList, FileDeletion};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Clone)]
pub struct FileCreateRequest {
    pub file: PathBuf,
    pub purpose: String,
}

pub struct FileRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> FileRequestBuilder<'a> {
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    pub async fn create(self, req: FileCreateRequest) -> Result<File, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_multipart("files", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn list(self) -> Result<FileList, GroqError> {
        let response = self.client.transport.get_json("files").await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn retrieve(self, file_id: String) -> Result<File, GroqError> {
        let path = format!("files/{}", file_id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn delete(self, file_id: String) -> Result<FileDeletion, GroqError> {
        let path = format!("files/{}", file_id);
        let response = self.client.transport.delete_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}