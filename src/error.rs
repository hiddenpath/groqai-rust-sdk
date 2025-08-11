// src/error.rs

use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Detailed error information from Groq API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqApiErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
    pub param: Option<String>,
}

/// Enhanced API error with structured details.
#[derive(Debug, Clone)]
pub struct GroqApiError {
    pub status: reqwest::StatusCode,
    pub details: Option<GroqApiErrorDetails>,
    pub raw_response: String,
}

impl std::fmt::Display for GroqApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API request failed with status code {}: {}", self.status, self.raw_response)?;
        if let Some(details) = &self.details {
            write!(f, " (Error: {}, Type: {})", 
                details.message, 
                details.error_type.as_deref().unwrap_or("unknown"))?;
        }
        Ok(())
    }
}

impl std::error::Error for GroqApiError {}

#[derive(Error, Debug)]
pub enum GroqError {
    #[error("{0}")]
    Api(#[from] GroqApiError),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Invalid message content: {0}")]
    InvalidMessage(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Groq API error: {0}")]
    GroqApiError(String),

    #[error("Multipart error: {0}")]
    Multipart(String),

    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    #[error("Stream parsing error: {0}")]
    StreamParsing(String),
}

impl GroqError {
    /// Create a new API error with structured details.
    pub fn api_error(status: reqwest::StatusCode, response_text: String) -> Self {
        let details = serde_json::from_str::<serde_json::Value>(&response_text)
            .ok()
            .and_then(|json| {
                json.get("error")
                    .and_then(|error| serde_json::from_value::<GroqApiErrorDetails>(error.clone()).ok())
            });

        let api_error = GroqApiError {
            status,
            details,
            raw_response: response_text,
        };

        GroqError::Api(api_error)
    }

    /// Check if this is a rate limit error.
    pub fn is_rate_limit(&self) -> bool {
        match self {
            GroqError::Api(api_error) => {
                api_error.status == reqwest::StatusCode::TOO_MANY_REQUESTS
                    || api_error.details.as_ref()
                        .map(|d| d.error_type.as_deref() == Some("rate_limit_exceeded"))
                        .unwrap_or(false)
            }
            _ => false,
        }
    }

    /// Check if this is an authentication error.
    pub fn is_authentication_error(&self) -> bool {
        match self {
            GroqError::Api(api_error) => {
                api_error.status == reqwest::StatusCode::UNAUTHORIZED
                    || api_error.status == reqwest::StatusCode::FORBIDDEN
            }
            _ => false,
        }
    }
}

