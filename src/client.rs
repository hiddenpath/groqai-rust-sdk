//! Client implementation for Groq API
//! 
//! 客户端实现模块，提供 Groq API 的主要接口

use std::sync::Arc;
use std::time::Duration;
use std::pin::Pin;
use futures::Stream;

use backoff::future::{Retry, Sleeper};
use tokio::time::{self, Sleep};
use tracing::instrument;
use url::Url;

use crate::api::chat::{ChatCompletionRequest, ChatRequestBuilder};
use crate::error::GroqError;
use crate::types::{ChatCompletionResponse, ChatCompletionChunk};
use crate::rate_limit::RateLimiter;
use crate::transport::{ApiKey, HttpTransport, Transport};

#[derive(Debug, Clone)]
struct TokioSleeper;

impl Sleeper for TokioSleeper {
    type Sleep = Sleep;

    fn sleep(&self, duration: Duration) -> Self::Sleep {
        time::sleep(duration)
    }
}

/// The main client for interacting with the Groq API.
/// 
/// `GroqClient` provides access to all Groq API endpoints including chat completions,
/// audio processing, file management, batch operations, and model information.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::{GroqClientBuilder, ChatMessage, Role};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
///         .build()?;
///     
///     // Use chat completions
///     let messages = vec![ChatMessage::new_text(Role::User, "Hello!")];
///     let response = client.chat("llama-3.1-70b-versatile")
///         .messages(messages)
///         .send()
///         .await?;
///     
///     println!("{}", response.choices[0].message.content);
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct GroqClient {
    pub transport: Arc<dyn Transport>,
    pub rate_limiter: RateLimiter,
    pub default_timeout: Duration,
}

/// Builder for creating a `GroqClient` instance.
/// 
/// The builder pattern allows for flexible configuration of the client
/// before creating the final instance.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::GroqClientBuilder;
/// use std::time::Duration;
/// 
/// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
///     .timeout(Duration::from_secs(60))
///     .build()?;
/// # Ok::<(), groqai::GroqError>(())
/// ```
pub struct GroqClientBuilder {
    api_key: ApiKey,
    base_url: Url,
    timeout: Duration,
    rate_limiter: RateLimiter,
    proxy: Option<reqwest::Proxy>,
}

impl GroqClientBuilder {
    /// Creates a new `GroqClientBuilder` with the provided API key.
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - A valid Groq API key that starts with "gsk_"
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError::InvalidApiKey` if the API key format is invalid.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// let builder = GroqClientBuilder::new("gsk_your_api_key".to_string())?;
    /// # Ok::<(), groqai::GroqError>(())
    /// ```
    pub fn new(api_key: String) -> Result<Self, GroqError> {
        let api_key = ApiKey::new(api_key)?;
        Ok(Self {
            api_key,
            base_url: Url::parse("https://api.groq.com/openai/v1/")?,
            timeout: Duration::from_secs(30),
            rate_limiter: RateLimiter::new(),
            proxy: None,
        })
    }

    /// Sets a custom base URL for the API.
    /// 
    /// # Arguments
    /// 
    /// * `url` - The base URL to use for API requests
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// use url::Url;
    /// 
    /// let builder = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    ///     .base_url(Url::parse("https://custom.api.endpoint/")?);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn base_url(mut self, url: Url) -> Self {
        self.base_url = url;
        self
    }

    /// Sets the request timeout duration.
    /// 
    /// # Arguments
    /// 
    /// * `duration` - The timeout duration for HTTP requests
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// use std::time::Duration;
    /// 
    /// let builder = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    ///     .timeout(Duration::from_secs(60));
    /// # Ok::<(), groqai::GroqError>(())
    /// ```
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Sets a proxy for HTTP requests.
    /// 
    /// # Arguments
    /// 
    /// * `proxy` - The proxy configuration to use
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// let proxy = reqwest::Proxy::http("http://proxy.example.com:8080")?;
    /// let builder = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    ///     .proxy(proxy);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn proxy(mut self, proxy: reqwest::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Builds the final `GroqClient` instance.
    /// 
    /// # Errors
    /// 
    /// Returns a `GroqError` if the client cannot be created due to
    /// configuration issues or network problems.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    ///     .build()?;
    /// # Ok::<(), groqai::GroqError>(())
    /// ```
    pub fn build(self) -> Result<GroqClient, GroqError> {
        let transport = HttpTransport::new(self.base_url, self.api_key, self.timeout, self.proxy)?;
        Ok(GroqClient {
            transport: Arc::new(transport),
            rate_limiter: self.rate_limiter,
            default_timeout: self.timeout,
        })
    }
}

impl GroqClient {
    /// Create a client using environment variables.
    ///
    /// Required:
    /// - GROQ_API_KEY
    ///
    /// Optional:
    /// - GROQ_PROXY_URL / HTTPS_PROXY / HTTP_PROXY
    /// - GROQ_TIMEOUT_SECS (default: 30)
    pub fn from_env() -> Result<Self, GroqError> {
        let api_key = std::env::var("GROQ_API_KEY")
            .map_err(|_| GroqError::InvalidApiKey("GROQ_API_KEY not set".into()))?;

        let mut builder = GroqClientBuilder::new(api_key)?;

        if let Ok(proxy_url) = std::env::var("GROQ_PROXY_URL")
            .or_else(|_| std::env::var("HTTPS_PROXY"))
            .or_else(|_| std::env::var("HTTP_PROXY"))
        {
            if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                builder = builder.proxy(proxy);
            }
        }

        let timeout_secs: u64 = std::env::var("GROQ_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        builder = builder.timeout(std::time::Duration::from_secs(timeout_secs));

        builder.build()
    }

    /// Create a client from a given API key with default settings.
    pub fn with_api_key(api_key: impl Into<String>) -> Result<Self, GroqError> {
        GroqClientBuilder::new(api_key.into())?
            .timeout(std::time::Duration::from_secs(30))
            .build()
    }

    /// Alias for `from_env()`.
    pub fn new() -> Result<Self, GroqError> {
        Self::from_env()
    }

    /// Creates a chat completion request builder.
    /// 
    /// # Arguments
    /// 
    /// * `model` - The model to use for chat completion
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{GroqClientBuilder, ChatMessage, Role};
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// let messages = vec![ChatMessage::new_text(Role::User, "Hello!")];
    /// 
    /// let response = client.chat("llama-3.1-70b-versatile")
    ///     .messages(messages)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn chat<'a>(&'a self, model: impl Into<String>) -> ChatRequestBuilder<'a> {
        ChatRequestBuilder::new(self, model)
    }

    /// Creates an audio processing request builder.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{GroqClientBuilder, AudioTranscriptionRequest};
    /// use std::path::PathBuf;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let request = AudioTranscriptionRequest {
    ///     file: Some(PathBuf::from("audio.mp3")),
    ///     url: None,
    ///     model: "whisper-large-v3".to_string(),
    ///     language: None,
    ///     prompt: None,
    ///     response_format: None,
    ///     temperature: None,
    ///     timestamp_granularities: None,
    /// };
    /// 
    /// let response = client.audio().transcribe(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn audio<'a>(&'a self) -> crate::api::audio::AudioRequestBuilder<'a> {
        crate::api::audio::AudioRequestBuilder::new(self)
    }

    /// Creates a batch processing request builder.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{GroqClientBuilder, BatchCreateRequest};
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let request = BatchCreateRequest {
    ///     input_file_id: "file_123".to_string(),
    ///     endpoint: "/chat/completions".to_string(),
    ///     completion_window: "24h".to_string(),
    ///     metadata: None,
    /// };
    /// 
    /// let batch = client.batches().create(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn batches<'a>(&'a self) -> crate::api::batches::BatchRequestBuilder<'a> {
        crate::api::batches::BatchRequestBuilder::new(self)
    }

    /// Creates a file management request builder.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let files = client.files().list().await?;
    /// println!("Found {} files", files.data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn files<'a>(&'a self) -> crate::api::files::FileRequestBuilder<'a> {
        crate::api::files::FileRequestBuilder::new(self)
    }

    /// Creates a models information request builder.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let models = client.models().list().await?;
    /// for model in models.data {
    ///     println!("Model: {}", model.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn models<'a>(&'a self) -> crate::api::models::ModelsRequestBuilder<'a> {
        crate::api::models::ModelsRequestBuilder::new(self)
    }

    /// Sends a chat completion request with retry logic.
    /// 
    /// This method includes built-in rate limiting and retry mechanisms
    /// for handling transient errors.
    /// 
    /// # Arguments
    /// 
    /// * `request` - The chat completion request to send
    /// 
    /// # Errors
    /// 
    /// Returns various `GroqError` types depending on the failure mode.
    #[instrument(skip(self, request), fields(model = %request.model))]
    pub async fn chat_completions(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, GroqError> {
        let op = || async {
            let res = self.transport.post_chat("chat/completions", &request).await;
            match res {
                Ok(response) => Ok(response),
                Err(GroqError::Api(api_err))
                    if api_err.status == reqwest::StatusCode::TOO_MANY_REQUESTS =>
                {
                    Err(backoff::Error::Transient {
                        err: GroqError::RateLimited,
                        retry_after: api_err.retry_after,
                    })
                }
                Err(e) => Err(backoff::Error::Permanent(e)),
            }
        };
        let notify = |_: GroqError, _: Duration| {};
        Retry::new(TokioSleeper, self.rate_limiter.backoff.clone(), notify, op)
            .await
            .map_err(GroqError::from)
    }

    /// Sends a streaming chat completion request.
    /// 
    /// Returns a stream of chat completion chunks for real-time processing.
    /// 
    /// # Arguments
    /// 
    /// * `request` - The chat completion request to send
    /// 
    /// # Returns
    /// 
    /// A stream of `ChatCompletionChunk` items or errors.
    #[instrument(skip(self, request), fields(model = %request.model))]
    pub async fn chat_completions_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError> {
        let url = self.transport.base_url().join("chat/completions")?;
        self.transport.post_stream(url, &request).await
    }
}