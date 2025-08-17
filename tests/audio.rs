use groqai::client::GroqClientBuilder;
use groqai::error::GroqError;
use groqai::api::audio::{AudioTranscriptionRequest, AudioTranslationRequest};
use std::path::PathBuf;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_audio_transcription_success() -> Result<(), GroqError> {
    let mock = MockServer::start().await;
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .base_url(mock.uri().parse().unwrap())
        .build()?;

    Mock::given(method("POST"))
        .and(path("/audio/transcriptions"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({"text": "Hello, world!"})))
        .mount(&mock)
        .await;

    let req = AudioTranscriptionRequest {
        file: None,
        url: Some("https://example.com/audio.mp3".to_string()),
        model: "whisper-large-v3".to_string(),
        language: None,
        prompt: None,
        response_format: None,
        temperature: None,
        timestamp_granularities: None,
    };

    let response = client.audio().transcribe(req).await?;
    assert_eq!(response.text, "Hello, world!");
    Ok(())
}

#[tokio::test]
async fn test_audio_transcription_rate_limit() -> Result<(), GroqError> {
    let mock = MockServer::start().await;
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .base_url(mock.uri().parse().unwrap())
        .build()?;

    Mock::given(method("POST"))
        .and(path("/audio/transcriptions"))
        .respond_with(ResponseTemplate::new(429)
            .append_header("retry-after", "1")
            .set_body_json(serde_json::json!({"error": {"message": "Rate limit exceeded", "type": "rate_limit_exceeded"}})))
        .mount(&mock)
        .await;

    let req = AudioTranscriptionRequest {
        file: None,
        url: Some("https://example.com/audio.mp3".to_string()),
        model: "whisper-large-v3".to_string(),
        language: None,
        prompt: None,
        response_format: None,
        temperature: None,
        timestamp_granularities: None,
    };

    let result = client.audio().transcribe(req).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_audio_translation_success() -> Result<(), GroqError> {
    let mock = MockServer::start().await;
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .base_url(mock.uri().parse().unwrap())
        .build()?;

    Mock::given(method("POST"))
        .and(path("/audio/translations"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({"text": "Translated text"})))
        .mount(&mock)
        .await;

    let req = AudioTranslationRequest {
        file: None,
        url: Some("https://example.com/audio.mp3".to_string()),
        model: "whisper-large-v3".to_string(),
        prompt: None,
        response_format: None,
        temperature: None,
    };

    let response = client.audio().translate(req).await?;
    assert_eq!(response.text, "Translated text");
    Ok(())
}

#[tokio::test]
async fn test_audio_transcription_invalid_file() -> Result<(), GroqError> {
    let client = GroqClientBuilder::new("gsk_test_key".to_string())
        .unwrap()
        .build()?;
    
    let req = AudioTranscriptionRequest {
        file: Some(PathBuf::from("test.txt")),
        url: None,
        model: "whisper-large-v3".to_string(),
        language: None,
        prompt: None,
        response_format: None,
        temperature: None,
        timestamp_granularities: None,
    };

    let result = client.audio().transcribe(req).await;
    assert!(result.is_err());
    Ok(())
}