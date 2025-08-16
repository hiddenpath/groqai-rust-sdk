pub mod api;
pub mod client;
pub mod error;
pub mod types;
pub mod rate_limit;
pub mod transport;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{GroqClient, GroqClientBuilder};
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

// 统一导出公共API
pub use api::chat::{ChatCompletionRequest, ChatRequestBuilder};
pub use client::GroqClient;
pub use client::GroqClientBuilder;
pub use error::GroqError;
pub use types::*;
pub use api::audio::{AudioTranscriptionRequest, AudioTranslationRequest, AudioRequestBuilder};
pub use api::batches::{BatchCreateRequest, BatchRequestBuilder};
pub use api::files::{FileCreateRequest, FileRequestBuilder};
pub use api::models::{ModelsRequestBuilder};
pub use api::fine_tunings::{FineTuningCreateRequest, FineTuningRequestBuilder};


