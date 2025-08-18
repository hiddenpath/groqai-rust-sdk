use groqai::{GroqClient, GroqError};

#[tokio::test]
async fn test_with_api_key() {
    let result = GroqClient::with_api_key("gsk_test_key_12345");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_with_api_key_invalid() {
    let result = GroqClient::with_api_key("invalid_key");
    assert!(result.is_err());
    
    if let Err(GroqError::InvalidApiKey(_)) = result {
        // Expected error
    } else {
        panic!("Expected InvalidApiKey error");
    }
}

#[tokio::test]
async fn test_from_env_with_valid_key() {
    std::env::set_var("GROQ_API_KEY", "gsk_test_key_12345");
    let result = GroqClient::from_env();
    assert!(result.is_ok());
}

#[tokio::test] 
async fn test_new_alias_with_valid_key() {
    std::env::set_var("GROQ_API_KEY", "gsk_test_key_12345");
    let result = GroqClient::new();
    assert!(result.is_ok());
}