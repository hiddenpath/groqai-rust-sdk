use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

// 包装 serde_json::Error
#[derive(Debug, Clone, Error)]
#[error("JSON serialization error: {0}")]
pub struct SerdeError(String);

impl From<serde_json::Error> for SerdeError {
    fn from(err: serde_json::Error) -> Self {
        SerdeError(err.to_string())
    }
}

// 包装 reqwest::Error
#[derive(Debug, Clone, Error)]
#[error("HTTP transport error: {0}")]
pub struct TransportError(String);

impl From<reqwest::Error> for TransportError {
    fn from(err: reqwest::Error) -> Self {
        TransportError(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqApiErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
    pub param: Option<String>,
}

#[derive(Debug, Clone, Error)]
#[error("API error {status}: {message}")]
pub struct GroqApiError {
    pub status: StatusCode,
    pub message: String,
    pub details: Option<GroqApiErrorDetails>,
    pub request_id: Option<String>,
    pub retry_after: Option<Duration>,
}

impl GroqApiError {
    pub fn from_response(status: StatusCode, raw: String, headers: &HeaderMap) -> Self {
        let details = serde_json::from_str::<GroqApiErrorDetails>(&raw).ok();
        let request_id = headers
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        let retry_after = headers
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);
        let message = details
            .as_ref()
            .map(|d| d.message.clone())
            .unwrap_or_else(|| status.canonical_reason().unwrap_or("API error").to_string());
        Self {
            status,
            message,
            details,
            request_id,
            retry_after,
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum GroqError {
    #[error("{0}")]
    Api(GroqApiError),

    #[error("{0}")]
    Serde(SerdeError),

    #[error("{0}")]
    Transport(TransportError),

    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    #[error("Invalid message content: {0}")]
    InvalidMessage(String),

    #[error("Multipart error: {0}")]
    Multipart(String),

    #[error("Stream parsing error: {0}")]
    StreamParsing(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Canceled")]
    Canceled,
}

impl From<serde_json::Error> for GroqError {
    fn from(err: serde_json::Error) -> Self {
        GroqError::Serde(SerdeError::from(err))
    }
}

impl From<reqwest::Error> for GroqError {
    fn from(err: reqwest::Error) -> Self {
        GroqError::Transport(TransportError::from(err))
    }
}

impl From<backoff::Error<GroqError>> for GroqError {
    fn from(error: backoff::Error<GroqError>) -> Self {
        match error {
            backoff::Error::Permanent(err) => err,
            backoff::Error::Transient { err, .. } => err,
        }
    }
}

impl From<url::ParseError> for GroqError {
    fn from(err: url::ParseError) -> Self {
        GroqError::InvalidMessage(format!("URL parse error: {}", err))
    }
}

impl GroqError {
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimited)
            || matches!(
                self,
                Self::Api(e) if e.status == StatusCode::TOO_MANY_REQUESTS
                    || e.details.as_ref().map(|d| d.error_type.as_deref() == Some("rate_limit_exceeded")).unwrap_or(false)
            )
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Api(e) if e.status.is_server_error() || e.status == StatusCode::TOO_MANY_REQUESTS
        ) || matches!(self, Self::Transport(_))
    }

    pub fn is_authentication_error(&self) -> bool {
        matches!(
            self,
            Self::Api(e) if e.status == StatusCode::UNAUTHORIZED || e.status == StatusCode::FORBIDDEN
        )
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Api(e) => e.retry_after,
            _ => None,
        }
    }
}
