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
//! ```rust,no_run
//! use groqai::{GroqClientBuilder, ChatMessage, Role};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = GroqClientBuilder::new("your-api-key".to_string())?
//!         .build()?;
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
//! ## API Key
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

// Public API exports
pub use api::chat::{ChatCompletionRequest, ChatRequestBuilder};
pub use client::{GroqClient, GroqClientBuilder};
pub use error::GroqError;
pub use types::*;
pub use api::audio::{AudioTranscriptionRequest, AudioTranslationRequest, AudioRequestBuilder};
pub use api::batches::{BatchCreateRequest, BatchRequestBuilder};
pub use api::files::{FileCreateRequest, FileRequestBuilder};
pub use api::models::ModelsRequestBuilder;
pub use api::fine_tunings::{FineTuningCreateRequest, FineTuningRequestBuilder};