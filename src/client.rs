use crate::error::GroqError;
use crate::models::{
    AudioTranscriptionRequest, AudioTranscriptionResponse, AudioTranslationRequest,
    AudioTranslationResponse, BatchListResponse, BatchObject, ChatCompletionChunk,
    ChatCompletionRequest, ChatCompletionResponse, FileDeleteResponse, FileListResponse, FileObject,
    Model, ModelListResponse, Tool, ToolChoice, ChatMessage,
};
use futures::stream::Stream;
use futures::TryStreamExt;
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use std::pin::Pin;

#[derive(Debug)]
/// A client for interacting with the Groq API.
///
/// This client provides methods for chat completions, file operations,
/// batch processing, and audio operations.
///
/// # Examples
///
/// ```rust,no_run
/// use groqai::GroqClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = GroqClient::new("gsk_your_api_key_here".to_string())?;
///
///     // Use the client...
///     Ok(())
/// }
/// ```
pub struct GroqClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl GroqClient {
    /// Create a new GroqClient with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Groq API key. Must start with "gsk_".
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the client if successful, or an error if the API key is invalid.
    ///
    /// # Errors
    ///
    /// * `InvalidApiKey` - If the API key is empty or has an invalid format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use groqai::GroqClient;
    ///
    /// let client = GroqClient::new("gsk_your_api_key_here".to_string())?;
    /// # Ok::<(), groqai::GroqError>(())
    /// ```
    pub fn new(api_key: String) -> Result<Self, GroqError> {
        if api_key.trim().is_empty() {
            return Err(GroqError::InvalidApiKey(
                "API key cannot be empty".to_string(),
            ));
        }

        // Basic validation: API key should start with "gsk_" for Groq
        if !api_key.starts_with("gsk_") {
            return Err(GroqError::InvalidApiKey(
                "Invalid API key format. Groq API keys should start with 'gsk_'".to_string(),
            ));
        }

        let client = Client::new();
        let base_url = "https://api.groq.com/openai/v1".to_string();
        Ok(GroqClient {
            base_url,
            api_key,
            client,
        })
    }

    /// Create a new GroqClient from the `GROQ_API_KEY` environment variable.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the client if successful, or an error if the environment variable is not set or invalid.
    ///
    /// # Errors
    ///
    /// * `InvalidApiKey` - If the `GROQ_API_KEY` environment variable is not set or contains an invalid API key.
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Set environment variable
    /// export GROQ_API_KEY="gsk_your_api_key_here"
    /// ```
    ///
    /// ```rust
    /// use groqai::GroqClient;
    ///
    /// let client = GroqClient::from_env()?;
    /// # Ok::<(), groqai::GroqError>(())
    /// ```
    pub fn from_env() -> Result<Self, GroqError> {
        let api_key = std::env::var("GROQ_API_KEY")
            .map_err(|_| GroqError::InvalidApiKey("GROQ_API_KEY environment variable not set".to_string()))?;
        
        Self::new(api_key)
    }

    async fn _send_request(&self, request_builder: RequestBuilder) -> Result<reqwest::Response, GroqError> {
        let response = request_builder
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }

        Ok(response)
    }

    async fn _get<T: DeserializeOwned>(&self, path: &str) -> Result<T, GroqError> {
        let url = format!("{}{}", self.base_url, path);
        let builder = self.client.get(&url);
        let response = self._send_request(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn _post_json<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, GroqError> {
        let url = format!("{}{}", self.base_url, path);
        let builder = self.client.post(&url).json(body);
        let response = self._send_request(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn _post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T, GroqError> {
        let url = format!("{}{}", self.base_url, path);
        let builder = self.client.post(&url);
        let response = self._send_request(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    async fn _delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, GroqError> {
        let url = format!("{}{}", self.base_url, path);
        let builder = self.client.delete(&url);
        let response = self._send_request(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    /// Chat completions
    pub async fn chat_completions(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, GroqError> {
        self._post_json("/chat/completions", &request).await
    }

    /// Streaming chat completions
    pub async fn stream_chat_completions(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError>
    {
        let url = format!("{}/chat/completions", self.base_url);
        let builder = self.client.post(&url).json(&request);
        let response = self._send_request(builder).await?;

        let bytes_stream = response.bytes_stream().map_err(GroqError::from);

        let stream = bytes_stream.try_filter_map(|chunk| async move {
            let chunk_str = match String::from_utf8(chunk.to_vec()) {
                Ok(s) => s,
                Err(_) => return Err(GroqError::StreamParsing("Invalid UTF-8 in stream chunk".to_string())),
            };

            // Process SSE format data
            for line in chunk_str.lines() {
                let line = line.trim();
                if line.is_empty() || !line.starts_with("data: ") {
                    continue;
                }

                let json_str = line.trim_start_matches("data: ");
                if json_str == "[DONE]" || json_str.trim().is_empty() {
                    continue;
                }

                match serde_json::from_str::<ChatCompletionChunk>(json_str) {
                    Ok(chunk) => return Ok(Some(chunk)),
                    Err(e) => {
                        return Err(GroqError::StreamParsing(format!("Failed to parse chunk JSON: {}", e)));
                    }
                }
            }

            Ok(None)
        });

        Ok(Box::pin(stream))
    }

    /// Get available models list
    pub async fn get_models(&self) -> Result<ModelListResponse, GroqError> {
        self._get("/models").await
    }

    /// Get model information
    pub async fn get_model(&self, model_id: &str) -> Result<Model, GroqError> {
        self._get(&format!("/models/{}", model_id)).await
    }

    /// Upload file
    pub async fn upload_file(&self, file_path: &str, purpose: &str) -> Result<FileObject, GroqError> {
        let url = format!("{}/files", self.base_url);
        let form = reqwest::multipart::Form::new()
            .text("purpose", purpose.to_string())
            .part(
                "file",
                reqwest::multipart::Part::file(file_path)
                    .await
                    .map_err(|e| GroqError::Multipart(e.to_string()))?,
            );
        
        let builder = self.client.post(&url).multipart(form);
        let response = self._send_request(builder).await?;

        response.json().await.map_err(GroqError::from)
    }

    /// List all files
    pub async fn list_files(&self) -> Result<FileListResponse, GroqError> {
        self._get("/files").await
    }

    /// Delete file
    pub async fn delete_file(&self, file_id: &str) -> Result<FileDeleteResponse, GroqError> {
        self._delete(&format!("/files/{}", file_id)).await
    }

    /// Get file information
    pub async fn retrieve_file(&self, file_id: &str) -> Result<FileObject, GroqError> {
        self._get(&format!("/files/{}", file_id)).await
    }

    /// Download file content
    pub async fn download_file(&self, file_id: &str) -> Result<bytes::Bytes, GroqError> {
        let url = format!("{}/files/{}/content", self.base_url, file_id);
        let builder = self.client.get(&url);
        let response = self._send_request(builder).await?;
        response.bytes().await.map_err(GroqError::from)
    }

    /// Create batch job
    pub async fn create_batch(
        &self,
        input_file_id: &str,
        completion_window: &str,
    ) -> Result<BatchObject, GroqError> {
        let body = serde_json::json!({
            "input_file_id": input_file_id,
            "endpoint": "/v1/chat/completions",
            "completion_window": completion_window
        });
        self._post_json("/batches", &body).await
    }

    /// Retrieve batch job
    pub async fn retrieve_batch(&self, batch_id: &str) -> Result<BatchObject, GroqError> {
        self._get(&format!("/batches/{}", batch_id)).await
    }

    /// Cancel batch job
    pub async fn cancel_batch(&self, batch_id: &str) -> Result<BatchObject, GroqError> {
        self._post_empty(&format!("/batches/{}/cancel", batch_id)).await
    }

    /// List all batch jobs
    pub async fn list_batches(&self) -> Result<BatchListResponse, GroqError> {
        self._get("/batches").await
    }

    /// Audio transcription
    pub async fn audio_transcription(
        &self,
        request: AudioTranscriptionRequest,
        file_path: &str,
    ) -> Result<AudioTranscriptionResponse, GroqError> {
        let url = format!("{}/audio/transcriptions", self.base_url);
        let mut form = reqwest::multipart::Form::new()
            .text("model", request.model)
            .part(
                "file",
                reqwest::multipart::Part::file(file_path)
                    .await
                    .map_err(|e| GroqError::Multipart(e.to_string()))?,
            );
        
        if let Some(lang) = request.language {
            form = form.text("language", lang);
        }
        if let Some(p) = request.prompt {
            form = form.text("prompt", p);
        }
        if let Some(fmt) = request.response_format {
            form = form.text("response_format", fmt);
        }
        if let Some(temp) = request.temperature {
            form = form.text("temperature", temp.to_string());
        }
        if let Some(gran) = request.timestamp_granularities {
            for g in gran {
                form = form.text("timestamp_granularities[]", g);
            }
        }
        
        let builder = self.client.post(&url).multipart(form);
        let response = self._send_request(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    /// Audio translation
    pub async fn audio_translation(
        &self,
        request: AudioTranslationRequest,
        file_path: &str,
    ) -> Result<AudioTranslationResponse, GroqError> {
        let url = format!("{}/audio/translations", self.base_url);
        let mut form = reqwest::multipart::Form::new()
            .text("model", request.model)
            .part(
                "file",
                reqwest::multipart::Part::file(file_path)
                    .await
                    .map_err(|e| GroqError::Multipart(e.to_string()))?,
            );
        
        if let Some(p) = request.prompt {
            form = form.text("prompt", p);
        }
        if let Some(fmt) = request.response_format {
            form = form.text("response_format", fmt);
        }
        if let Some(temp) = request.temperature {
            form = form.text("temperature", temp.to_string());
        }
        if let Some(lang) = request.language {
            form = form.text("language", lang);
        }
        
        let builder = self.client.post(&url).multipart(form);
        let response = self._send_request(builder).await?;
        response.json().await.map_err(GroqError::from)
    }

    /// Audio speech synthesis
    pub async fn audio_speech(
        &self,
        model: &str,
        input: &str,
        voice: &str,
        response_format: Option<&str>,
        sample_rate: Option<u32>,
        speed: Option<f32>,
    ) -> Result<bytes::Bytes, GroqError> {
        let url = format!("{}/audio/speech", self.base_url);
        let mut body = serde_json::json!({
            "model": model,
            "input": input,
            "voice": voice
        });
        if let Some(fmt) = response_format {
            body["response_format"] = serde_json::json!(fmt);
        }
        if let Some(rate) = sample_rate {
            body["sample_rate"] = serde_json::json!(rate);
        }
        if let Some(s) = speed {
            body["speed"] = serde_json::json!(s);
        }

        let builder = self.client.post(&url).json(&body);
        let response = self._send_request(builder).await?;
        response.bytes().await.map_err(GroqError::from)
    }

    /// Helper method to create a tool call request
    pub fn create_tool_call_request(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
        tools: Vec<Tool>,
        tool_choice: Option<ToolChoice>,
    ) -> ChatCompletionRequest {
        ChatCompletionRequest {
            messages,
            model: model.to_string(),
            tools: Some(tools),
            tool_choice,
            ..Default::default()
        }
    }

    /// Helper method to handle tool calls in chat completion
    pub async fn chat_with_tools(
        &self,
        messages: Vec<ChatMessage>,
        model: &str,
        tools: Vec<Tool>,
        tool_choice: Option<ToolChoice>,
    ) -> Result<ChatCompletionResponse, GroqError> {
        let request = self.create_tool_call_request(messages, model, tools, tool_choice);
        self.chat_completions(request).await
    }
}