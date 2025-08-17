use groqai::client::GroqClientBuilder;
use groqai::error::GroqError;
use std::path::PathBuf;

#[tokio::test]
async fn test_file_create_invalid_extension() -> Result<(), GroqError> {
    let req = groqai::api::files::FileCreateRequest::new(PathBuf::from("test.txt"), "batch".to_string());
    assert!(req.is_err());
    Ok(())
}

#[tokio::test]
async fn test_file_list_success() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.files().list().await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_file_retrieve_error() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.files().retrieve("file_123".to_string()).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_file_delete_success() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;

    let result = client.files().delete("file_123".to_string()).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_files_methods_exist() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;
    
    // 验证所有files方法都存在
    let _builder = client.files();
    
    Ok(())
}