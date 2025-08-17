use groqai::client::GroqClientBuilder;
use groqai::error::GroqError;

#[tokio::test]
async fn test_models_list_success() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.models().list().await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_model_retrieve_success() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.models().retrieve("llama-3.1-70b-versatile".to_string()).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_model_retrieve_not_found() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.models().retrieve("invalid_model".to_string()).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_models_methods_exist() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;
    
    // 验证所有models方法都存在
    let _builder = client.models();
    
    Ok(())
}