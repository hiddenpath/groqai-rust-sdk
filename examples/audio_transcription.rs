// examples/audio_transcription.rs
// Audio transcription and translation example

use groqai::{GroqClient, AudioTranscriptionRequest, AudioTranslationRequest};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using environment variables (recommended)
    let client = GroqClient::new()?;
    
    // Transcription example
    let transcription_request = AudioTranscriptionRequest {
        file: Some(PathBuf::from("audio.mp3")),
        url: None,
        model: "whisper-large-v3".to_string(),
        language: Some("en".to_string()),
        prompt: None,
        response_format: Some("json".to_string()),
        temperature: Some(0.0),
        timestamp_granularities: None,
    };
    
    match client.audio().transcribe(transcription_request).await {
        Ok(transcription) => println!("Transcription: {}", transcription.text),
        Err(e) => println!("Transcription failed: {}", e),
    }
    
    // Translation example
    let translation_request = AudioTranslationRequest {
        file: Some(PathBuf::from("spanish_audio.mp3")),
        url: None,
        model: "whisper-large-v3".to_string(),
        prompt: None,
        response_format: Some("json".to_string()),
        temperature: Some(0.0),
    };
    
    match client.audio().translate(translation_request).await {
        Ok(translation) => println!("Translation: {}", translation.text),
        Err(e) => println!("Translation failed: {}", e),
    }
    
    Ok(())
}