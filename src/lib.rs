//! # GroqAI - Rust Client Library for Groq API
//! 
//! GroqAI 是一个用于 Groq API 的 Rust 客户端库
//! 
//! This crate provides a comprehensive Rust client for interacting with the Groq API,
//! offering support for chat completions, audio transcription/translation, file management,
//! batch processing, and model information retrieval.
//! 
//! ## Features
//! 
//! - **Chat Completions**: Support for both streaming and non-streaming chat completions
//! - **Audio Processing**: Audio transcription and translation capabilities
//! - **File Management**: Upload, list, retrieve, and delete files
//! - **Batch Processing**: Create and manage batch jobs for efficient processing
//! - **Model Information**: Retrieve available models and their details
//! - **Rate Limiting**: Built-in rate limiting and retry mechanisms
//! - **Error Handling**: Comprehensive error types for robust error handling
//! 
//! ## Quick Start
//! 
//! ### Using Environment Variables (Recommended)
//! 
//! **⚠️ Required: Set `GROQ_API_KEY` environment variable first!**
//! 
//! ```bash
//! export GROQ_API_KEY="gsk_your_api_key_here"
//! ```
//! 
//! ```rust,no_run
//! // Option 1: Import specific types
//! use groqai::{GroqClient, ChatMessage, Role};
//! 
//! // Option 2: Use prelude for convenience (imports most common types)
//! // use groqai::prelude::*;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Creates client from GROQ_API_KEY environment variable
//!     let client = GroqClient::new()?;
//!     
//!     let messages = vec![
//!         ChatMessage::new_text(Role::User, "Hello, how are you?")
//!     ];
//!     
//!     let response = client
//!         .chat("llama-3.1-70b-versatile")
//!         .messages(messages)
//!         .send()
//!         .await?;
//!     
//!     println!("Response: {}", response.choices[0].message.content);
//!     Ok(())
//! }
//! ```
//! 
//! ### Using API Key Directly
//! 
//! ```rust,no_run
//! use groqai::prelude::*;  // Convenient import
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = GroqClient::with_api_key("gsk_your_api_key")?;
//!     
//!     let messages = vec![
//!         ChatMessage::new_text(Role::User, "Hello!")
//!     ];
//!     
//!     let response = client
//!         .chat("llama-3.1-70b-versatile")
//!         .messages(messages)
//!         .send()
//!         .await?;
//!     
//!     println!("Response: {}", response.choices[0].message.content);
//!     Ok(())
//! }
//! ```
//! 
//! ## Configuration
//! 
//! ### Environment Variables
//! 
//! ```bash
//! # Required
//! export GROQ_API_KEY="gsk_your_api_key_here"
//! 
//! # Optional
//! export GROQ_PROXY_URL="http://proxy.example.com:8080"
//! export GROQ_TIMEOUT_SECS="60"  # default: 30
//! ```
//! 
//! ### API Key
//! 
//! You need a valid Groq API key to use this library. The API key must start with "gsk_".
//! You can obtain one from the [Groq Console](https://console.groq.com/).

pub mod api;
pub mod client;
pub mod error;
pub mod types;
pub mod rate_limit;
pub mod transport;

#[cfg(test)]
mod tests {
    use crate::client::GroqClientBuilder;
    use crate::error::GroqError;
    use crate::types::{ChatMessage, Role};

    #[test]
    fn test_client_builder() {
        let client = GroqClientBuilder::new("gsk_test123".to_string())
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(client.default_timeout, std::time::Duration::from_secs(30));
    }

    #[test]
    fn test_invalid_api_key() {
        let result = GroqClientBuilder::new("invalid".to_string());
        assert!(matches!(result, Err(GroqError::InvalidApiKey(_))));
    }

    #[test]
    fn test_chat_message_serde() {
        let msg = ChatMessage::new_text(Role::User, "Hello");
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: ChatMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg.content, deserialized.content);
    }
}

// ============================================================================
// Public API Exports - Organized for Developer Convenience
// ============================================================================

// Core Client (Most Important - Users need these first)
pub use client::{GroqClient, GroqClientBuilder};
pub use error::GroqError;

// Essential Types (Common usage)
pub use types::{
    // Message types (most commonly used)
    ChatMessage, Role, MessageContent, MessagePart, ImageUrl,
    // Model types
    KnownModel,
    // Response types
    ChatCompletionResponse, Choice, Usage,
    ChatCompletionChunk, ChoiceChunk, MessageDelta,
};

// Request Builders (Fluent API)
pub use api::chat::ChatRequestBuilder;
pub use api::audio::AudioRequestBuilder;
pub use api::files::FileRequestBuilder;
pub use api::batches::BatchRequestBuilder;
pub use api::models::ModelsRequestBuilder;
pub use api::fine_tunings::FineTuningRequestBuilder;

// Request Types (For advanced usage)
pub use api::chat::ChatCompletionRequest;
pub use api::audio::{AudioTranscriptionRequest, AudioTranslationRequest};
pub use api::files::FileCreateRequest;
pub use api::batches::BatchCreateRequest;
pub use api::fine_tunings::FineTuningCreateRequest;

// Response Types (For advanced usage)
pub use types::{
    // Audio responses
    Transcription, Translation,
    // File responses
    WorkFile, WorkFileList, WorkFileDeletion,
    // Model responses
    Model, ModelList,
    // Batch responses
    Batch, BatchList, RequestCounts,
    // Advanced types
    Tool, ToolCall, FunctionCall, FunctionDef,
    ResponseFormat, ToolChoice, ServiceTier, StopSequence,
    StreamOptions, CompoundCustom, SearchSettings,
};

// ============================================================================
// Convenience Re-exports for Common Patterns
// ============================================================================

/// Prelude module for convenient imports
/// 
/// ```rust
/// use groqai::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        GroqClient, GroqError,
        ChatMessage, Role, MessageContent,
        KnownModel,
        ChatCompletionResponse,
    };
}