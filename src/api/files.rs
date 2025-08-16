use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{WorkFile, WorkFileList, WorkFileDeletion};
use serde::Serialize;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Serialize, Clone)]
pub struct FileCreateRequest {
    pub file: PathBuf,
    pub purpose: String,
}

impl FileCreateRequest {
    pub fn new(file: PathBuf, purpose: String) -> Result<Self, GroqError> {
        // 验证文件扩展名
        if file.extension().and_then(|ext| ext.to_str()) != Some("jsonl") {
            return Err(GroqError::InvalidMessage(
                "File must have .jsonl extension".to_string(),
            ));
        }

        // 验证文件内容（每行是有效 JSON）
        let file_reader = File::open(&file)
            .map_err(|e| GroqError::InvalidMessage(format!("Failed to open file: {}", e)))?;
        let reader = BufReader::new(file_reader);
        for (index, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| {
                GroqError::InvalidMessage(format!("Failed to read line {}: {}", index + 1, e))
            })?;
            if !line.trim().is_empty() {
                serde_json::from_str::<serde_json::Value>(&line).map_err(|e| {
                    GroqError::InvalidMessage(format!("Invalid JSONL at line {}: {}", index + 1, e))
                })?;
            }
        }

        Ok(Self { file, purpose })
    }
}

pub struct FileRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> FileRequestBuilder<'a> {
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    pub async fn create(self, req: FileCreateRequest) -> Result<WorkFile, GroqError> {
        let body = serde_json::to_value(&req)?;
        let response = self.client.transport.post_multipart("files", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn list(self) -> Result<WorkFileList, GroqError> {
        let response = self.client.transport.get_json("files").await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn retrieve(self, file_id: String) -> Result<WorkFile, GroqError> {
        let path = format!("files/{}", file_id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn delete(self, file_id: String) -> Result<WorkFileDeletion, GroqError> {
        let path = format!("files/{}", file_id);
        let response = self.client.transport.delete_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}