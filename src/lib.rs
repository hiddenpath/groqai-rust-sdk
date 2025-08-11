// Declare and export our modules
pub mod client;
pub mod error;
pub mod models;

#[cfg(test)]
mod tests {

    use crate::client::GroqClient;
    use crate::error::GroqError;
    use crate::models::{ChatMessage, Role, ChatCompletionRequest};

    #[test]
    fn test_groq_client_new_with_valid_key() {
        let result = GroqClient::new("gsk_test123456789".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_groq_client_new_with_empty_key() {
        let result = GroqClient::new("".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            GroqError::InvalidApiKey(_) => {},
            _ => panic!("Expected InvalidApiKey error"),
        }
    }

    #[test]
    fn test_groq_client_new_with_invalid_format() {
        let result = GroqClient::new("invalid_key".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            GroqError::InvalidApiKey(_) => {},
            _ => panic!("Expected InvalidApiKey error"),
        }
    }

    #[test]
    fn test_chat_message_creation() {
        let message = ChatMessage::new_text(Role::User, "Hello, world!".to_string());
        assert_eq!(message.role, Role::User);
        match message.content {
            crate::models::MessageContent::Text(text) => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_chat_completion_request_default() {
        let request = ChatCompletionRequest::default();
        assert_eq!(request.model, "llama3-8b-8192");
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_completion_tokens, Some(1000));
    }
}

pub use client::GroqClient;
pub use error::GroqError;

pub use models::{
    Choice, Usage, ModelListResponse, Model, Permission, Delta, ChatCompletionChunk, ChunkChoice,
    FileObject, FileListResponse, FileDeleteResponse,
    BatchObject, BatchListResponse, BatchRequestCounts,
    AudioTranscriptionResponse, AudioTranslationResponse,
    ResponseFormat, Tool, ToolChoice, FunctionDef,
    MessageContent, MessagePart, ImageUrl, ToolCall, FunctionCall,
    AudioTranscriptionRequest, AudioTranslationRequest,
};
