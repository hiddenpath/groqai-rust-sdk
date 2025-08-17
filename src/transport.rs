//! HTTP transport layer for Groq API communication
//! 
//! 传输层模块，处理与 Groq API 的 HTTP 通信

use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use reqwest::multipart::{Form, Part};
use reqwest::{Client, RequestBuilder};
use std::pin::Pin;
use std::time::Duration;
use tracing::debug;
use url::Url;

use crate::api::chat::ChatCompletionRequest;
use crate::error::{GroqApiError, GroqError};
use crate::types::{ChatCompletionChunk, ChatCompletionResponse};

/// 流式数据缓冲区，用于处理不完整的SSE数据
struct StreamBuffer {
    buffer: String,
    consecutive_errors: u32,
    max_consecutive_errors: u32,
}

impl StreamBuffer {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            consecutive_errors: 0,
            max_consecutive_errors: 5,
        }
    }

    fn add_bytes(&mut self, bytes: &[u8]) {
        self.buffer.push_str(&String::from_utf8_lossy(bytes));
    }

    fn process_lines(&mut self) -> Vec<Result<ChatCompletionChunk, GroqError>> {
        let mut chunks = Vec::new();

        // 检查是否有换行符
        if !self.buffer.contains('\n') {
            return chunks; // 没有完整的行
        }

        // 找到最后一个换行符的位置
        let last_newline = self.buffer.rfind('\n').unwrap();

        // 处理完整的行（不包括最后一行）
        let complete_lines = &self.buffer[..last_newline];
        let remaining = &self.buffer[last_newline + 1..];

        // 处理完整的行
        for line in complete_lines.lines() {
            if line.starts_with("data: ") && !line.ends_with("[DONE]") {
                let json = line.strip_prefix("data: ").unwrap_or(line);
                match serde_json::from_str::<ChatCompletionChunk>(json) {
                    Ok(chunk) => {
                        chunks.push(Ok(chunk));
                        self.consecutive_errors = 0; // 重置错误计数
                    }
                    Err(e) => {
                        self.consecutive_errors += 1;
                        debug!(
                            "Failed to parse chunk (error {}): {}",
                            self.consecutive_errors, e
                        );

                        // 尝试处理部分数据
                        if let Some(partial_chunk) = self.try_recover_partial_chunk(json) {
                            chunks.push(partial_chunk);
                        }

                        // 如果连续错误过多，记录但继续处理
                        if self.consecutive_errors >= self.max_consecutive_errors {
                            debug!("Too many consecutive parsing errors, but continuing...");
                        }
                    }
                }
            }
        }

        // 更新缓冲区，保留不完整的行
        self.buffer = remaining.to_string();

        chunks
    }

    fn try_recover_partial_chunk(
        &self,
        json: &str,
    ) -> Option<Result<ChatCompletionChunk, GroqError>> {
        // 尝试修复常见的JSON格式问题
        let mut fixed_json = json.to_string();

        // 修复未闭合的字符串
        if fixed_json.matches('"').count() % 2 == 1 {
            fixed_json.push('"');
        }

        // 修复未闭合的对象
        if fixed_json.matches('{').count() > fixed_json.matches('}').count() {
            let missing_braces = fixed_json.matches('{').count() - fixed_json.matches('}').count();
            fixed_json.push_str(&"}".repeat(missing_braces));
        }

        // 尝试解析修复后的JSON
        match serde_json::from_str::<ChatCompletionChunk>(&fixed_json) {
            Ok(chunk) => {
                debug!("Successfully recovered partial chunk");
                Some(Ok(chunk))
            }
            Err(_) => {
                // 如果仍然失败，不存储部分数据（避免借用问题）
                None
            }
        }
    }
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn post_chat(
        &self,
        path: &str,
        body: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, GroqError>;

    async fn post_stream(
        &self,
        url: Url,
        body: &ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError>;

    async fn post_stream_with_retry(
        &self,
        url: Url,
        body: &ChatCompletionRequest,
        max_retries: u32,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError>;

    async fn post_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, GroqError>;

    async fn post_multipart(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, GroqError>;

    async fn get_json(&self, path: &str) -> Result<serde_json::Value, GroqError>;

    async fn get_with_params(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<serde_json::Value, GroqError>;

    async fn delete_json(&self, path: &str) -> Result<serde_json::Value, GroqError>;

    // 批处理相关方法
    async fn post_batch_create(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, GroqError>;
    
    async fn get_batch_retrieve(
        &self,
        batch_id: &str,
    ) -> Result<serde_json::Value, GroqError>;
    
    async fn get_batch_list(
        &self,
        params: &[(&str, String)],
    ) -> Result<serde_json::Value, GroqError>;
    
    async fn post_batch_cancel(
        &self,
        batch_id: &str,
    ) -> Result<serde_json::Value, GroqError>;

    fn base_url(&self) -> &Url;
}

pub struct HttpTransport {
    client: Client,
    base_url: Url,
    api_key: ApiKey,
}

impl HttpTransport {
    pub fn new(
        base_url: Url,
        api_key: ApiKey,
        timeout: Duration,
        proxy: Option<reqwest::Proxy>,
    ) -> Result<Self, GroqError> {
        let mut builder = Client::builder().timeout(timeout);
        if let Some(p) = proxy {
            builder = builder.proxy(p);
        }
        let client = builder.build()?;
        Ok(Self {
            client,
            base_url,
            api_key,
        })
    }

    async fn send(&self, builder: RequestBuilder) -> Result<reqwest::Response, GroqError> {
        debug!("Sending request: {:?}", builder);
        let response = builder
            .header("Authorization", format!("Bearer {}", self.api_key.0))
            .send()
            .await
            .map_err(GroqError::from)?;
        debug!(
            "Response status: {}, headers: {:?}",
            response.status(),
            response.headers()
        );
        if !response.status().is_success() {
            let headers = response.headers().clone();
            let status = response.status();
            let text = response.text().await?;
            debug!("Error response body: {}", text);
            return Err(GroqError::Api(GroqApiError::from_response(
                status, text, &headers,
            )));
        }
        Ok(response)
    }

    async fn build_multipart(body: &serde_json::Value) -> Result<Form, GroqError> {
        let mut form = Form::new();

        if let Some(url) = body["url"].as_str() {
            form = form.part("url", Part::text(url.to_string()));
        }

        if let Some(file_path) = body["file"].as_str() {
            let part = Part::file(file_path).await.map_err(|e| GroqError::InvalidMessage(format!("File error: {}", e)))?;
            form = form.part("file", part);
        }

        if let Some(model) = body["model"].as_str() {
            form = form.part("model", Part::text(model.to_string()));
        }

        if let Some(language) = body["language"].as_str() {
            form = form.part("language", Part::text(language.to_string()));
        }

        if let Some(prompt) = body["prompt"].as_str() {
            form = form.part("prompt", Part::text(prompt.to_string()));
        }

        if let Some(response_format) = body["response_format"].as_str() {
            form = form.part("response_format", Part::text(response_format.to_string()));
        }

        if let Some(temperature) = body["temperature"].as_f64() {
            form = form.part("temperature", Part::text(temperature.to_string()));
        }

        Ok(form)
    }

    async fn attempt_stream_request(
        &self,
        url: Url,
        body: &ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError>
    {
        let mut request = body.clone();
        request.stream = Some(true);
        let builder = self.client.post(url).json(&request);
        let response = self.send(builder).await?;

        // 改进的流式处理：使用map_with进行状态管理
        let mut buffer = StreamBuffer::new();
        let stream = response
            .bytes_stream()
            .map_err(GroqError::from)
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        // 将新字节添加到缓冲区
                        buffer.add_bytes(&bytes);

                        // 处理完整的行
                        let chunks = buffer.process_lines();

                        if chunks.is_empty() {
                            futures::stream::iter(vec![])
                        } else {
                            futures::stream::iter(chunks)
                        }
                    }
                    Err(e) => {
                        // 记录错误但继续处理
                        debug!("Stream bytes error: {:?}", e);
                        futures::stream::iter(vec![Err(GroqError::from(e))])
                    }
                }
            })
            .flatten()
            .filter_map(|result| async move {
                match result {
                    Ok(chunk) => Some(Ok(chunk)),
                    Err(e) => {
                        // 对于解析错误，记录但不中断流
                        debug!("Chunk parsing error: {:?}", e);
                        None
                    }
                }
            });

        Ok(Box::pin(stream))
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn post_chat(
        &self,
        path: &str,
        body: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, GroqError> {
        let url = self
            .base_url
            .join(path)
            .map_err(|e| GroqError::InvalidMessage(format!("URL parse error: {}", e)))?;
        let builder = self.client.post(url).json(body);
        let response = self.send(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn post_stream(
        &self,
        url: Url,
        body: &ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError>
    {
        self.post_stream_with_retry(url, body, 0).await
    }

    async fn post_stream_with_retry(
        &self,
        url: Url,
        body: &ChatCompletionRequest,
        max_retries: u32,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError>
    {
        let mut retry_count = 0;
        let mut last_error = None;

        while retry_count <= max_retries {
            match self.attempt_stream_request(url.clone(), body).await {
                Ok(stream) => {
                    debug!("Stream request successful after {} retries", retry_count);
                    return Ok(stream);
                }
                Err(e) => {
                    last_error = Some(e.clone());
                    retry_count += 1;

                    if retry_count <= max_retries {
                        debug!(
                            "Stream request failed (attempt {}/{}), retrying...",
                            retry_count, max_retries
                        );
                        // 指数退避重试
                        let delay = Duration::from_millis(100 * 2_u64.pow(retry_count as u32));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            GroqError::InvalidMessage("Max retries exceeded for stream request".to_string())
        }))
    }

    async fn post_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, GroqError> {
        let url = self
            .base_url
            .join(path)
            .map_err(|e| GroqError::InvalidMessage(format!("URL parse error: {}", e)))?;
        let builder = self.client.post(url).json(body);
        let response = self.send(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn post_multipart(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, GroqError> {
        let url = self.base_url.join(path)?;
        let form = Self::build_multipart(body).await?;
        let builder = self.client.post(url).multipart(form);
        let response = self.send(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn get_json(&self, path: &str) -> Result<serde_json::Value, GroqError> {
        let url = self.base_url.join(path)?;
        let builder = self.client.get(url);
        let response = self.send(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn get_with_params(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<serde_json::Value, GroqError> {
        let url = self.base_url.join(path)?;
        let mut url_builder = self.client.get(url);
        for (key, value) in params {
            url_builder = url_builder.query(&[(*key, value)]);
        }
        let response = self.send(url_builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn delete_json(&self, path: &str) -> Result<serde_json::Value, GroqError> {
        let url = self.base_url.join(path)?;
        let builder = self.client.delete(url);
        let response = self.send(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn post_batch_create(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, GroqError> {
        self.post_json("batches", body).await
    }
    
    async fn get_batch_retrieve(
        &self,
        batch_id: &str,
    ) -> Result<serde_json::Value, GroqError> {
        self.get_json(&format!("batches/{}", batch_id)).await
    }
    
    async fn get_batch_list(
        &self,
        params: &[(&str, String)],
    ) -> Result<serde_json::Value, GroqError> {
        self.get_with_params("batches", params).await
    }
    
    async fn post_batch_cancel(
        &self,
        batch_id: &str,
    ) -> Result<serde_json::Value, GroqError> {
        let empty_body = serde_json::json!({});
        self.post_json(&format!("batches/{}/cancel", batch_id), &empty_body).await
    }

    fn base_url(&self) -> &Url {
        &self.base_url
    }
}

#[derive(Clone)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(key: String) -> Result<Self, GroqError> {
        let trimmed = key.trim();
        if trimmed.is_empty() || !trimmed.starts_with("gsk_") {
            return Err(GroqError::InvalidApiKey(
                "Invalid API key format".to_string(),
            ));
        }
        Ok(Self(key))
    }
}

impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiKey(redacted)")
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiKey(redacted)")
    }
}
