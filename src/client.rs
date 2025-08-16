// client.rs
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

#[derive(Clone)]
pub struct GroqClient {
    pub transport: Arc<dyn Transport>,
    pub rate_limiter: RateLimiter,
    pub default_timeout: Duration,
}

pub struct GroqClientBuilder {
    api_key: ApiKey,
    base_url: Url,
    timeout: Duration,
    rate_limiter: RateLimiter,
    proxy: Option<reqwest::Proxy>,
}

impl GroqClientBuilder {
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

    pub fn base_url(mut self, url: Url) -> Self {
        self.base_url = url;
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    pub fn proxy(mut self, proxy: reqwest::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

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
    pub fn chat<'a>(&'a self, model: impl Into<String>) -> ChatRequestBuilder<'a> {
        ChatRequestBuilder::new(self, model)
    }

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

    #[instrument(skip(self, request), fields(model = %request.model))]
    pub async fn chat_completions_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError> {
        let url = self.transport.base_url().join("chat/completions")?;
        self.transport.post_stream(url, &request).await
    }
}