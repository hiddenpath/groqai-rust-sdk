//! Chat completion API implementation
//! 
//! 聊天完成 API 实现，支持流式和非流式对话

use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{
    ChatCompletionResponse, ChatCompletionChunk, ChatMessage, Tool, ToolChoice,
    ResponseFormat, ServiceTier, StopSequence, StreamOptions, CompoundCustom, SearchSettings
};
use serde::Serialize;
use std::pin::Pin;
use futures::Stream;

/// Request structure for chat completions
/// 
/// This struct contains all the parameters that can be sent to the chat completions endpoint.
/// Most fields are optional and have sensible defaults.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::api::chat::ChatCompletionRequest;
/// use groqai::types::{ChatMessage, Role};
/// 
/// let request = ChatCompletionRequest {
///     messages: vec![ChatMessage::new_text(Role::User, "Hello!")],
///     model: "llama-3.1-70b-versatile".to_string(),
///     temperature: Some(0.7),
///     max_completion_tokens: Some(1000),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletionRequest {
    /// List of messages in the conversation
    pub messages: Vec<ChatMessage>,
    /// Model to use for the completion
    pub model: String,
    /// Sampling temperature between 0 and 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    /// List of tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Controls which tool the model should use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Frequency penalty between -2.0 and 2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Presence penalty between -2.0 and 2.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Whether to return log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Number of most likely tokens to return at each position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    /// Modify likelihood of specified tokens appearing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::HashMap<String, f32>>,
    /// Whether to enable parallel tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Reasoning effort level for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    /// Search settings for web search capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_settings: Option<SearchSettings>,
    /// Format of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Number of completions to generate (currently only supports 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Random seed for deterministic outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    /// Service tier to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    /// Stop sequences to end generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequence>,
    /// Options for streaming responses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    /// Custom compound settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compound_custom: Option<CompoundCustom>,
}

/// Builder for creating chat completion requests
/// 
/// This builder provides a fluent interface for constructing chat completion requests
/// with various parameters and options.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::{GroqClientBuilder, ChatMessage, Role};
/// 
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
/// 
/// let response = client.chat("llama-3.1-70b-versatile")
///     .message(ChatMessage::new_text(Role::User, "Hello!"))
///     .temperature(0.8)
///     .max_completion_tokens(500)
///     .send()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct ChatRequestBuilder<'a> { 
    client: &'a GroqClient,
    request: ChatCompletionRequest,
    stream: bool,
}

impl<'a> ChatRequestBuilder<'a> {
    /// Creates a new chat request builder
    /// 
    /// # Arguments
    /// 
    /// * `client` - Reference to the GroqClient
    /// * `model` - The model to use for completion
    pub fn new(client: &'a GroqClient, model: impl Into<String>) -> Self {
        Self {
            client,
            request: ChatCompletionRequest {
                model: model.into(),
                messages: Vec::new(),
                temperature: Some(0.7),
                max_completion_tokens: Some(1000),
                ..Default::default()
            },
            stream: false,
        }
    }

    /// Adds a single message to the conversation
    /// 
    /// # Arguments
    /// 
    /// * `msg` - The message to add
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{ChatMessage, Role};
    /// # use groqai::GroqClientBuilder;
    /// # let client = GroqClientBuilder::new("gsk_key".to_string()).unwrap().build().unwrap();
    /// 
    /// let builder = client.chat("llama-3.1-70b-versatile")
    ///     .message(ChatMessage::new_text(Role::User, "Hello!"));
    /// ```
    pub fn message(mut self, msg: ChatMessage) -> Self {
        self.request.messages.push(msg);
        self
    }

    /// Sets multiple messages for the conversation
    /// 
    /// # Arguments
    /// 
    /// * `messages` - Vector of messages
    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.request.messages = messages;
        self
    }

    /// Sets the tools available to the model
    /// 
    /// # Arguments
    /// 
    /// * `tools` - Vector of tools the model can use
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    /// Sets the tool choice strategy
    /// 
    /// # Arguments
    /// 
    /// * `choice` - How the model should choose tools
    pub fn tool_choice(mut self, choice: ToolChoice) -> Self {
        self.request.tool_choice = Some(choice);
        self
    }

    /// Sets the sampling temperature
    /// 
    /// # Arguments
    /// 
    /// * `temp` - Temperature between 0.0 and 2.0
    pub fn temperature(mut self, temp: f32) -> Self {
        self.request.temperature = Some(temp);
        self
    }

    /// Sets the maximum number of completion tokens
    /// 
    /// # Arguments
    /// 
    /// * `max_tokens` - Maximum tokens to generate
    pub fn max_completion_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_completion_tokens = Some(max_tokens);
        self
    }

    /// Sets the frequency penalty
    /// 
    /// # Arguments
    /// 
    /// * `penalty` - Penalty between -2.0 and 2.0
    pub fn frequency_penalty(mut self, penalty: f32) -> Self {
        self.request.frequency_penalty = Some(penalty);
        self
    }

    /// Sets the presence penalty
    /// 
    /// # Arguments
    /// 
    /// * `penalty` - Penalty between -2.0 and 2.0
    pub fn presence_penalty(mut self, penalty: f32) -> Self {
        self.request.presence_penalty = Some(penalty);
        self
    }

    /// Enables or disables log probabilities
    /// 
    /// # Arguments
    /// 
    /// * `logprobs` - Whether to return log probabilities
    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.request.logprobs = Some(logprobs);
        self
    }

    /// Sets the number of top log probabilities to return
    /// 
    /// # Arguments
    /// 
    /// * `top_logprobs` - Number of top log probabilities
    pub fn top_logprobs(mut self, top_logprobs: i32) -> Self {
        self.request.top_logprobs = Some(top_logprobs);
        self
    }

    /// Sets logit bias for specific tokens
    /// 
    /// # Arguments
    /// 
    /// * `logit_bias` - Map of token IDs to bias values
    pub fn logit_bias(mut self, logit_bias: std::collections::HashMap<String, f32>) -> Self {
        self.request.logit_bias = Some(logit_bias);
        self
    }

    /// Enables or disables parallel tool calls
    /// 
    /// # Arguments
    /// 
    /// * `parallel_tool_calls` - Whether to allow parallel tool calls
    pub fn parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.request.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }

    /// Sets the response format
    /// 
    /// # Arguments
    /// 
    /// * `format` - The desired response format
    pub fn response_format(mut self, format: ResponseFormat) -> Self {
        self.request.response_format = Some(format);
        self
    }

    /// Sets the reasoning effort level
    /// 
    /// # Arguments
    /// 
    /// * `reasoning_effort` - The reasoning effort level
    pub fn reasoning_effort(mut self, reasoning_effort: String) -> Self {
        self.request.reasoning_effort = Some(reasoning_effort);
        self
    }

    /// Sets search settings for web search capabilities
    /// 
    /// # Arguments
    /// 
    /// * `search_settings` - Search configuration
    pub fn search_settings(mut self, search_settings: SearchSettings) -> Self {
        self.request.search_settings = Some(search_settings);
        self
    }

    /// Sets the number of completions to generate
    /// 
    /// # Arguments
    /// 
    /// * `n` - Number of completions (currently only 1 is supported)
    pub fn n(mut self, n: u32) -> Self {
        self.request.n = Some(n);
        self
    }

    /// Sets a random seed for deterministic outputs
    /// 
    /// # Arguments
    /// 
    /// * `seed` - Random seed value
    pub fn seed(mut self, seed: i32) -> Self {
        self.request.seed = Some(seed);
        self
    }

    /// Sets the service tier
    /// 
    /// # Arguments
    /// 
    /// * `service_tier` - The service tier to use
    pub fn service_tier(mut self, service_tier: ServiceTier) -> Self {
        self.request.service_tier = Some(service_tier);
        self
    }

    /// Sets stop sequences
    /// 
    /// # Arguments
    /// 
    /// * `stop` - Stop sequences to end generation
    pub fn stop(mut self, stop: StopSequence) -> Self {
        self.request.stop = Some(stop);
        self
    }

    /// Sets streaming options
    /// 
    /// # Arguments
    /// 
    /// * `stream_options` - Options for streaming responses
    pub fn stream_options(mut self, stream_options: StreamOptions) -> Self {
        self.request.stream_options = Some(stream_options);
        self
    }

    /// Sets compound custom settings
    /// 
    /// # Arguments
    /// 
    /// * `compound_custom` - Custom compound settings
    pub fn compound_custom(mut self, compound_custom: CompoundCustom) -> Self {
        self.request.compound_custom = Some(compound_custom);
        self
    }

    /// Enables or disables streaming
    /// 
    /// # Arguments
    /// 
    /// * `enable` - Whether to enable streaming
    pub fn stream(mut self, enable: bool) -> Self {
        self.stream = enable;
        self.request.stream = Some(enable);
        self
    }

    /// Sends the chat completion request
    /// 
    /// # Returns
    /// 
    /// A `ChatCompletionResponse` containing the model's response
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the request fails or if streaming is enabled
    /// (use `send_stream()` for streaming requests)
    /// 
    /// # Panics
    /// 
    /// Panics if streaming is enabled. Use `send_stream()` instead.
    pub async fn send(self) -> Result<ChatCompletionResponse, GroqError> {
        if self.stream {
            panic!("Use send_stream() for streaming requests");
        }
        self.client.chat_completions(self.request).await
    }

    /// Sends a streaming chat completion request
    /// 
    /// # Returns
    /// 
    /// A stream of `ChatCompletionChunk` items
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the request fails or if streaming is disabled
    /// (use `send()` for non-streaming requests)
    /// 
    /// # Panics
    /// 
    /// Panics if streaming is disabled. Use `send()` instead.
    pub async fn send_stream(self) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError> {
        if !self.stream {
            panic!("Use send() for non-streaming requests");
        }
        self.client.chat_completions_stream(self.request).await
    }
}