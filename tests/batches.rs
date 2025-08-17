use groqai::client::GroqClientBuilder;
use groqai::error::GroqError;
use groqai::api::batches::BatchCreateRequest;

#[tokio::test]
async fn test_batch_create_success() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let req = BatchCreateRequest {
        input_file_id: "file_123".to_string(),
        endpoint: "/chat/completions".to_string(),
        completion_window: "24h".to_string(),
        metadata: None,
    };

    let result = client.batches().create(req).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_batch_retrieve_success() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.batches().retrieve("batch_123".to_string()).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_batch_list_with_params() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.batches().list(Some("batch_123".to_string()), Some(10)).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_batch_cancel_error() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.batches().cancel("batch_123".to_string()).await;
    assert!(result.is_err());
    Ok(())
}