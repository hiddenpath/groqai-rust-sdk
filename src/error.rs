//! Error types and handling for the Groq API client
//! 
//! 错误类型和处理模块，定义了所有可能的错误情况

use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Wrapper for JSON serialization errors
#[derive(Debug, Clone, Error)]
#[error("JSON serialization error: {0}")]
pub struct SerdeError(String);

impl From<serde_json::Error> for SerdeError {
    fn from(err: serde_json::Error) -> Self {
        SerdeError(err.to_string())
    }
}

/// Wrapper for HTTP transport errors
#[derive(Debug, Clone, Error)]
#[error("HTTP transport error: {0}")]
pub struct TransportError(String);

impl From<reqwest::Error> for TransportError {
    fn from(err: reqwest::Error) -> Self {
        TransportError(err.to_string())
    }
}

/// Details of an API error response from Groq
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqApiErrorDetails {
    /// The error message from the API
    pub message: String,
    /// The type of error (e.g., "invalid_request_error", "rate_limit_exceeded")
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    /// Additional error code if provided
    pub code: Option<String>,
    /// Parameter that caused the error, if applicable
    pub param: Option<String>,
}

/// API error response structure from Groq
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqApiError {
    /// HTTP status code of the error response
    #[serde(skip)]
    pub status: StatusCode,
    /// Detailed error information
    pub error: GroqApiErrorDetails,
    /// Retry-After header value for rate limiting, if present
    #[serde(skip)]
    pub retry_after: Option<Duration>,
}

impl GroqApiError {
    /// Creates a new API error from response components
    /// 
    /// # Arguments
    /// 
    /// * `status` - HTTP status code
    /// * `body` - Response body as string
    /// * `headers` - HTTP response headers
    pub fn from_response(status: StatusCode, body: String, headers: &HeaderMap) -> Self {
        let retry_after = headers
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        let error_details = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v.get("error").cloned())
            .and_then(|e| serde_json::from_value(e).ok())
            .unwrap_or_else(|| GroqApiErrorDetails {
                message: body,
                error_type: None,
                code: None,
                param: None,
            });

        Self {
            status,
            error: error_details,
            retry_after,
        }
    }
}

impl std::fmt::Display for GroqApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Groq API error ({}): {}", self.status, self.error.message)
    }
}

impl std::error::Error for GroqApiError {}

/// Main error type for the Groq client library
/// 
/// This enum covers all possible error conditions that can occur
/// when using the Groq API client.
#[derive(Debug, Clone, Error)]
pub enum GroqError {
    /// Invalid API key format or authentication failure
    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    /// Invalid message content or request parameters
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Rate limiting error - too many requests
    #[error("Rate limited - too many requests")]
    RateLimited,

    /// API returned an error response
    #[error("API error: {0}")]
    Api(#[from] GroqApiError),

    /// HTTP transport layer error
    #[error("Transport error: {0}")]
    Transport(#[from] TransportError),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serde(#[from] SerdeError),

    /// URL parsing error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Backoff/retry mechanism error
    #[error("Backoff error: {0}")]
    Backoff(String),
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

impl<E> From<backoff::Error<E>> for GroqError
where
    E: Into<GroqError>,
{
    fn from(err: backoff::Error<E>) -> Self {
        match err {
            backoff::Error::Permanent(e) => e.into(),
            backoff::Error::Transient { err, .. } => err.into(),
        }
    }
}

impl GroqError {
    /// Returns true if this error is retryable
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use groqai::GroqError;
    /// 
    /// let rate_limit_error = GroqError::RateLimited;
    /// assert!(rate_limit_error.is_retryable());
    /// 
    /// let invalid_key_error = GroqError::InvalidApiKey("bad key".to_string());
    /// assert!(!invalid_key_error.is_retryable());
    /// ```
    pub fn is_retryable(&self) -> bool {
        matches!(self, GroqError::RateLimited | GroqError::Transport(_))
    }

    /// Returns true if this is a rate limiting error
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use groqai::GroqError;
    /// 
    /// let rate_limit_error = GroqError::RateLimited;
    /// assert!(rate_limit_error.is_rate_limited());
    /// ```
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, GroqError::RateLimited)
    }

    /// Returns true if this is an authentication error
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use groqai::GroqError;
    /// 
    /// let auth_error = GroqError::InvalidApiKey("bad key".to_string());
    /// assert!(auth_error.is_auth_error());
    /// ```
    pub fn is_auth_error(&self) -> bool {
        matches!(self, GroqError::InvalidApiKey(_))
    }
}