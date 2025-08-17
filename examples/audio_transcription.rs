// examples/audio_transcription.rs
// Audio transcription and translation example

use groqai::{GroqClientBuilder, AudioTranscriptionRequest, AudioTranslationRequest};
use std::{env, path::PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    
    let client = GroqClientBuilder::new(api_key)?.build()?;
    
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