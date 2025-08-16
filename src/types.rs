use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownModel {
    Llama3_1_8bInstant,
    Llama3_1_70bVersatile,
    Llama3_1_405bReasoning,
    Mixtral8x7b32768,
    Gemma2_9bIt,
    Qwen2_5_72bInstruct,
    Other(String),
}

impl From<KnownModel> for String {
    fn from(model: KnownModel) -> Self {
        match model {
            KnownModel::Llama3_1_8bInstant => "llama-3.1-8b-instant".to_string(),
            KnownModel::Llama3_1_70bVersatile => "llama-3.1-70b-versatile".to_string(),
            KnownModel::Llama3_1_405bReasoning => "llama-3.1-405b-reasoning".to_string(),
            KnownModel::Mixtral8x7b32768 => "mixtral-8x7b-32768".to_string(),
            KnownModel::Gemma2_9bIt => "gemma2-9b-it".to_string(),
            KnownModel::Qwen2_5_72bInstruct => "qwen2.5-72b-instruct".to_string(),
            KnownModel::Other(s) => s,
        }
    }
}

impl fmt::Display for KnownModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

#[derive(PartialEq, Serialize, Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    ImageUrl(ImageUrl),
    Parts(Vec<MessagePart>),
}

impl MessageContent {
    pub fn text(content: impl Into<String>) -> Self {
        Self::Text(content.into())
    }
    pub fn image(url: impl Into<String>) -> Self {
        Self::ImageUrl(ImageUrl::new(url))
    }
    pub fn parts(parts: Vec<MessagePart>) -> Self {
        Self::Parts(parts)
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum MessagePart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl ImageUrl {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            detail: None,
        }
    }
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionCall,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: Role,
    pub content: MessageContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    pub fn new_text(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: MessageContent::Text(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn new_multimodal(role: Role, parts: Vec<MessagePart>) -> Self {
        Self {
            role,
            content: MessageContent::Parts(parts),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn tool_response(tool_call_id: String, content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: MessageContent::Text(content.into()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Tool {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionDef,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct FunctionDef {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ToolChoice {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)] // 添加 Clone
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

#[derive(Deserialize, Debug, Clone)] // 添加 Clone
pub struct Choice {
    pub index: u32,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

#[derive(Deserialize, Debug, Clone)] // 添加 Clone
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug, Clone)] // 添加 Clone
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChoiceChunk>,
    pub system_fingerprint: Option<String>,
}

#[derive(Deserialize, Debug, Clone)] // 添加 Clone
pub struct ChoiceChunk {
    pub index: i32,
    pub delta: MessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug, Clone)] // 添加 Clone
pub struct MessageDelta {
    pub role: Option<Role>,
    pub content: Option<MessageContent>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

// 现有内容...

#[derive(Deserialize, Debug, Clone)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub active: bool,
    pub context_window: u32,
    pub public_apps: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ModelList {
    pub object: String,
    pub data: Vec<Model>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transcription {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_groq: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Translation {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_groq: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct File {
    pub id: String,
    pub object: String,
    pub bytes: u64,
    pub created_at: u64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileList {
    pub object: String,
    pub data: Vec<File>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileDeletion {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Batch {
    pub id: String,
    pub object: String,
    pub endpoint: String,
    pub errors: Option<serde_json::Value>,
    pub input_file_id: String,
    pub completion_window: String,
    pub status: String,
    pub output_file_id: Option<String>,
    pub error_file_id: Option<String>,
    pub created_at: u64,
    pub in_progress_at: Option<u64>,
    pub expires_at: u64,
    pub finalizing_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub failed_at: Option<u64>,
    pub expired_at: Option<u64>,
    pub cancelling_at: Option<u64>,
    pub cancelled_at: Option<u64>,
    pub request_counts: RequestCounts,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCounts {
    pub total: u32,
    pub completed: u32,
    pub failed: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BatchList {
    pub object: String,
    pub data: Vec<Batch>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {
    Auto,
    OnDemand,
    Flex,
    Performance,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StopSequence {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StreamOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CompoundCustom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SearchSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,
}