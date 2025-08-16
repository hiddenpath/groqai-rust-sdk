use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{
    ChatCompletionResponse, ChatCompletionChunk, ChatMessage, Tool, ToolChoice,
    ResponseFormat, ServiceTier, StopSequence, StreamOptions, CompoundCustom, SearchSettings
};
use serde::Serialize;
use std::pin::Pin;
use futures::Stream;

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletionRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::HashMap<String, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_settings: Option<SearchSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    // 新增参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>, // 当前只支持1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compound_custom: Option<CompoundCustom>,
}


#[derive(Clone)]
pub struct ChatRequestBuilder<'a> { 
    client: &'a GroqClient,
    request: ChatCompletionRequest,
    stream: bool, // 添加 stream 字段
}

impl<'a> ChatRequestBuilder<'a> {
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

    pub fn message(mut self, msg: ChatMessage) -> Self {
        self.request.messages.push(msg);
        self
    }

    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    pub fn tool_choice(mut self, choice: ToolChoice) -> Self {
        self.request.tool_choice = Some(choice);
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.request.temperature = Some(temp);
        self
    }

    pub fn frequency_penalty(mut self, penalty: f32) -> Self {
        self.request.frequency_penalty = Some(penalty);
        self
    }

    pub fn presence_penalty(mut self, penalty: f32) -> Self {
        self.request.presence_penalty = Some(penalty);
        self
    }

    pub fn logprobs(mut self, logprobs: bool) -> Self {
        self.request.logprobs = Some(logprobs);
        self
    }

    pub fn top_logprobs(mut self, top_logprobs: i32) -> Self {
        self.request.top_logprobs = Some(top_logprobs);
        self
    }

    pub fn logit_bias(mut self, logit_bias: std::collections::HashMap<String, f32>) -> Self {
        self.request.logit_bias = Some(logit_bias);
        self
    }

    pub fn parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.request.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }

    pub fn response_format(mut self, format: ResponseFormat) -> Self { // <-- Add this method
        self.request.response_format = Some(format);
        self
    }

    pub fn reasoning_effort(mut self, reasoning_effort: String) -> Self {
        self.request.reasoning_effort = Some(reasoning_effort);
        self
    }

    pub fn search_settings(mut self, search_settings: SearchSettings) -> Self {
        self.request.search_settings = Some(search_settings);
        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.request.n = Some(n);
        self
    }

    pub fn seed(mut self, seed: i32) -> Self {
        self.request.seed = Some(seed);
        self
    }

    pub fn service_tier(mut self, service_tier: ServiceTier) -> Self {
        self.request.service_tier = Some(service_tier);
        self
    }

    pub fn stop(mut self, stop: StopSequence) -> Self {
        self.request.stop = Some(stop);
        self
    }

    pub fn stream_options(mut self, stream_options: StreamOptions) -> Self {
        self.request.stream_options = Some(stream_options);
        self
    }

    pub fn compound_custom(mut self, compound_custom: CompoundCustom) -> Self {
        self.request.compound_custom = Some(compound_custom);
        self
    }

    pub fn stream(mut self, enable: bool) -> Self {
        self.stream = enable;
        self.request.stream = Some(enable);
        self
    }

    pub async fn send(self) -> Result<ChatCompletionResponse, GroqError> {
        if self.stream {
            panic!("Use send_stream() for streaming requests");
        }
        self.client.chat_completions(self.request).await
    }

    pub async fn send_stream(self) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionChunk, GroqError>> + Send>>, GroqError> {
        if !self.stream {
            panic!("Use send() for non-streaming requests");
        }
        self.client.chat_completions_stream(self.request).await
    }
}
