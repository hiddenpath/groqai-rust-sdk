use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{Transcription, Translation};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Clone)]
pub struct AudioTranscriptionRequest {
    pub file: Option<PathBuf>,
    pub url: Option<String>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<String>>,
}

#[derive(Serialize, Clone)]
pub struct AudioTranslationRequest {
    pub file: Option<PathBuf>,
    pub url: Option<String>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

pub struct AudioRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> AudioRequestBuilder<'a> {
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    pub async fn transcribe(self, req: AudioTranscriptionRequest) -> Result<Transcription, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_multipart("audio/transcriptions", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    pub async fn translate(self, req: AudioTranslationRequest) -> Result<Translation, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_multipart("audio/translations", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}