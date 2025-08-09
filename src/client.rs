use crate::error::GroqError;
use crate::models::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse,
    ModelListResponse, FileObject, FileListResponse, FileDeleteResponse,
    BatchObject, BatchListResponse, AudioTranscriptionResponse, AudioTranslationResponse,
    AudioTranscriptionRequest, AudioTranslationRequest, Tool, ToolChoice, ChatMessage,
};
use futures::TryStreamExt;
use futures::stream::Stream;
use reqwest::Client;
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
            return Err(GroqError::InvalidApiKey("API key cannot be empty".to_string()));
        }
        
        // Basic validation: API key should start with "gsk_" for Groq
        if !api_key.starts_with("gsk_") {
            return Err(GroqError::InvalidApiKey("Invalid API key format. Groq API keys should start with 'gsk_'".to_string()));
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

    /// Chat completions
    pub async fn chat_completions(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, GroqError> {
        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }

        let chat_response: ChatCompletionResponse = response.json().await?;
        Ok(chat_response)
    }

    /// Streaming chat completions
    pub async fn stream_chat_completions(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError>
    {
        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }

        let bytes_stream = response.bytes_stream().map_err(GroqError::from);

        let stream = bytes_stream.try_filter_map(|chunk| async move {
            let chunk_str = match String::from_utf8(chunk.to_vec()) {
                Ok(s) => s,
                Err(_) => return Err(GroqError::StreamParsing("Invalid UTF-8 in stream chunk".to_string())),
            };

            // Process SSE format data
            let lines: Vec<&str> = chunk_str.lines().collect();

            for line in lines {
                let line = line.trim();
                if line.is_empty() || line == "data: [DONE]" {
                    continue;
                }

                if line.starts_with("data: ") {
                    let json_str = line.trim_start_matches("data: ");
                    if json_str == "[DONE]" {
                        continue;
                    }

                    // Skip empty data lines
                    if json_str.trim().is_empty() {
                        continue;
                    }

                    match serde_json::from_str::<ChatCompletionChunk>(json_str) {
                        Ok(chunk) => return Ok(Some(chunk)),
                        Err(e) => {
                            // Log the invalid JSON for debugging but don't fail the entire stream
                            eprintln!("Failed to parse chunk JSON: {} from line: {}", e, json_str);
                            continue;
                        }
                    }
                }
            }

            Ok(None)
        });

        Ok(Box::pin(stream))
    }

    /// Get available models list
    pub async fn get_models(&self) -> Result<ModelListResponse, GroqError> {
        let url = format!("{}/models", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let models: ModelListResponse = response.json().await?;
        Ok(models)
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
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let file_object: FileObject = response.json().await?;
        Ok(file_object)
    }

    /// List all files
    pub async fn list_files(&self) -> Result<FileListResponse, GroqError> {
        let url = format!("{}/files", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let file_list: FileListResponse = response.json().await?;
        Ok(file_list)
    }

    /// Delete file
    pub async fn delete_file(&self, file_id: &str) -> Result<FileDeleteResponse, GroqError> {
        let url = format!("{}/files/{}", self.base_url, file_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let delete_response: FileDeleteResponse = response.json().await?;
        Ok(delete_response)
    }

    /// Get file information
    pub async fn retrieve_file(&self, file_id: &str) -> Result<FileObject, GroqError> {
        let url = format!("{}/files/{}", self.base_url, file_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let file_object: FileObject = response.json().await?;
        Ok(file_object)
    }

    /// Download file content
    pub async fn download_file(&self, file_id: &str) -> Result<bytes::Bytes, GroqError> {
        let url = format!("{}/files/{}/content", self.base_url, file_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        Ok(response.bytes().await?)
    }

    /// Create batch job
    pub async fn create_batch(
        &self,
        input_file_id: &str,
        completion_window: &str,
    ) -> Result<BatchObject, GroqError> {
        let url = format!("{}/batches", self.base_url);
        let body = serde_json::json!({
            "input_file_id": input_file_id,
            "endpoint": "/v1/chat/completions",
            "completion_window": completion_window
        });
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let batch_object: BatchObject = response.json().await?;
        Ok(batch_object)
    }

    /// Retrieve batch job
    pub async fn retrieve_batch(&self, batch_id: &str) -> Result<BatchObject, GroqError> {
        let url = format!("{}/batches/{}", self.base_url, batch_id);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let batch_object: BatchObject = response.json().await?;
        Ok(batch_object)
    }

    /// Cancel batch job
    pub async fn cancel_batch(&self, batch_id: &str) -> Result<BatchObject, GroqError> {
        let url = format!("{}/batches/{}/cancel", self.base_url, batch_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let batch_object: BatchObject = response.json().await?;
        Ok(batch_object)
    }

    /// List all batch jobs
    pub async fn list_batches(&self) -> Result<BatchListResponse, GroqError> {
        let url = format!("{}/batches", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let batch_list: BatchListResponse = response.json().await?;
        Ok(batch_list)
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
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let transcription: AudioTranscriptionResponse = response.json().await?;
        Ok(transcription)
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
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        let translation: AudioTranslationResponse = response.json().await?;
        Ok(translation)
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
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GroqError::api_error(status, text));
        }
        Ok(response.bytes().await?)
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
