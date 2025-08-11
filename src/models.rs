// file: src/models.rs

use serde::{Serialize, Deserialize};

// Role enum for message roles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

// Message content type supporting multimodal content
#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    MultiModal(Vec<MessagePart>),
}

// Message content parts supporting text and images
#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "text")]
    Text {
        text: String,
    },
    #[serde(rename = "image_url")]
    ImageUrl {
        image_url: ImageUrl,
    },
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

// Function call information
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

// Chat message struct containing role, content and tool calls
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: Role,
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

// Helper methods for ChatMessage
impl ChatMessage {
    /// Create a simple text message
    pub fn new_text(role: Role, content: String) -> Self {
        Self {
            role,
            content: MessageContent::Text(content),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a multimodal message
    pub fn new_multimodal(role: Role, parts: Vec<MessagePart>) -> Self {
        Self {
            role,
            content: MessageContent::MultiModal(parts),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a message with tool calls
    pub fn with_tool_calls(mut self, tool_calls: Vec<ToolCall>) -> Self {
        self.tool_calls = Some(tool_calls);
        self
    }

    /// Create a tool response message
    pub fn tool_response(tool_call_id: String, content: String) -> Self {
        Self {
            role: Role::Tool,
            content: MessageContent::Text(content),
            tool_calls: None,
            tool_call_id: Some(tool_call_id),
        }
    }
}

// Complete Chat Completion request structure
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct ChatCompletionRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_reasoning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

impl ChatCompletionRequest {
    pub fn default() -> Self {
        Self {
            messages: Vec::new(),
            model: "llama3-8b-8192".to_string(),
            temperature: Some(0.7),
            max_completion_tokens: Some(1000),
            stream: Some(false),
            tools: Some(Vec::new()),
            include_reasoning: Some(true),
            reasoning_effort: Some("low".to_string()),
            reasoning_format: Some("json".to_string()),
            top_logprobs: Some(10),
            ..Default::default()
        }
    }

    pub fn stop_with_single(mut self, stop: String) -> Self {
        self.stop = Some(vec![stop]);
        self
    }

    pub fn stop_with_multiple(mut self, stops: Vec<String>) -> Self {
        self.stop = Some(stops);
        self
    }

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ResponseFormat {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ToolChoice {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Tool {
    pub r#type: String, // 目前仅支持 "function"
    pub function: FunctionDef,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct FunctionDef {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: serde_json::Value, // JSON Schema
}

// Chat Completion response structure
#[derive(Deserialize, Debug)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_groq: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

/// Log probability information for tokens
#[derive(Serialize, Deserialize, Debug)]
pub struct LogProbs {
    pub content: Vec<LogProbContent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogProbContent {
    pub token: String,
    pub logprob: f64,
    pub bytes: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<TopLogProb>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct ModelListResponse {
    pub data: Vec<Model>,
}

#[derive(Deserialize, Debug)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub permission: Vec<Permission>,
}

#[derive(Deserialize, Debug)]
pub struct Permission {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Delta {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
}

#[derive(Deserialize, Debug)]
pub struct ChunkChoice {
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

// File-related structures
#[derive(Deserialize, Debug)]
pub struct FileObject {
    pub id: String,
    pub object: String,
    pub bytes: u64,
    pub created_at: u64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Deserialize, Debug)]
pub struct FileListResponse {
    pub object: String,
    pub data: Vec<FileObject>,
}

#[derive(Deserialize, Debug)]
pub struct FileDeleteResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

// Batch-related structures
#[derive(Deserialize, Debug)]
pub struct BatchObject {
    pub id: String,
    pub object: String,
    pub endpoint: String,
    pub input_file_id: String,
    pub completion_window: String,
    pub status: String,
    pub request_counts: Option<BatchRequestCounts>,
    pub created_at: u64,
    pub expires_at: u64,
    // Other fields can be added as needed
}

#[derive(Deserialize, Debug)]
pub struct BatchRequestCounts {
    pub total: u32,
    pub completed: u32,
    pub failed: u32,
}

#[derive(Deserialize, Debug)]
pub struct BatchListResponse {
    pub object: String,
    pub data: Vec<BatchObject>,
}

// Audio transcription/translation/speech synthesis related structures
#[derive(Deserialize, Debug)]
pub struct AudioTranscriptionResponse {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_groq: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct AudioTranslationResponse {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_groq: Option<serde_json::Value>,
}

// Audio transcription request structure
#[derive(Serialize, Debug)]
pub struct AudioTranscriptionRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<String>>,
}

impl AudioTranscriptionRequest {
    pub fn new(model: String) -> Self {
        Self {
            model,
            language: None,
            prompt: None,
            response_format: None,
            temperature: None,
            timestamp_granularities: None,
        }
    }

    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response_format(mut self, response_format: String) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn timestamp_granularities(mut self, granularities: Vec<String>) -> Self {
        self.timestamp_granularities = Some(granularities);
        self
    }
}

// Audio translation request structure
#[derive(Serialize, Debug)]
pub struct AudioTranslationRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl AudioTranslationRequest {
    pub fn new(model: String) -> Self {
        Self {
            model,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None,
        }
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response_format(mut self, response_format: String) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }
}

// Speech synthesis directly returns audio binary stream, no structure needed