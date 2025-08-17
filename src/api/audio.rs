//! Audio transcription and translation API implementation
//! 
//! 音频转录和翻译 API 实现，支持多种音频格式处理

use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{Transcription, Translation};
use serde::Serialize;
use std::path::PathBuf;

/// Request structure for audio transcription
/// 
/// This struct contains parameters for transcribing audio files to text.
/// You can provide either a file path or a URL to the audio content.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::api::audio::AudioTranscriptionRequest;
/// use std::path::PathBuf;
/// 
/// let request = AudioTranscriptionRequest {
///     file: Some(PathBuf::from("audio.mp3")),
///     url: None,
///     model: "whisper-large-v3".to_string(),
///     language: Some("en".to_string()),
///     prompt: None,
///     response_format: Some("json".to_string()),
///     temperature: Some(0.0),
///     timestamp_granularities: None,
/// };
/// ```
#[derive(Serialize, Clone)]
pub struct AudioTranscriptionRequest {
    /// Path to the audio file to transcribe
    pub file: Option<PathBuf>,
    /// URL to the audio file to transcribe
    pub url: Option<String>,
    /// Model to use for transcription (e.g., "whisper-large-v3")
    pub model: String,
    /// Language of the input audio (ISO-639-1 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Optional text prompt to guide the model's style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Format of the response (json, text, srt, verbose_json, vtt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Sampling temperature between 0 and 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Timestamp granularities (word, segment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<String>>,
}

/// Request structure for audio translation
/// 
/// This struct contains parameters for translating audio files to English text.
/// You can provide either a file path or a URL to the audio content.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::api::audio::AudioTranslationRequest;
/// use std::path::PathBuf;
/// 
/// let request = AudioTranslationRequest {
///     file: Some(PathBuf::from("spanish_audio.mp3")),
///     url: None,
///     model: "whisper-large-v3".to_string(),
///     prompt: None,
///     response_format: Some("json".to_string()),
///     temperature: Some(0.0),
/// };
/// ```
#[derive(Serialize, Clone)]
pub struct AudioTranslationRequest {
    /// Path to the audio file to translate
    pub file: Option<PathBuf>,
    /// URL to the audio file to translate
    pub url: Option<String>,
    /// Model to use for translation (e.g., "whisper-large-v3")
    pub model: String,
    /// Optional text prompt to guide the model's style
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Format of the response (json, text, srt, verbose_json, vtt)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Sampling temperature between 0 and 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

/// Builder for audio processing requests
/// 
/// This builder provides methods for transcribing and translating audio files
/// using Groq's Whisper models.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::{GroqClientBuilder, AudioTranscriptionRequest};
/// use std::path::PathBuf;
/// 
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
/// 
/// let request = AudioTranscriptionRequest {
///     file: Some(PathBuf::from("audio.mp3")),
///     url: None,
///     model: "whisper-large-v3".to_string(),
///     language: Some("en".to_string()),
///     prompt: None,
///     response_format: None,
///     temperature: None,
///     timestamp_granularities: None,
/// };
/// 
/// let transcription = client.audio().transcribe(request).await?;
/// println!("Transcription: {}", transcription.text);
/// # Ok(())
/// # }
/// ```
pub struct AudioRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> AudioRequestBuilder<'a> {
    /// Creates a new audio request builder
    /// 
    /// # Arguments
    /// 
    /// * `client` - Reference to the GroqClient
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    /// Transcribes audio to text
    /// 
    /// # Arguments
    /// 
    /// * `req` - The transcription request parameters
    /// 
    /// # Returns
    /// 
    /// A `Transcription` containing the transcribed text and metadata
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the transcription fails
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{GroqClientBuilder, AudioTranscriptionRequest};
    /// use std::path::PathBuf;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let request = AudioTranscriptionRequest {
    ///     file: Some(PathBuf::from("meeting.mp3")),
    ///     url: None,
    ///     model: "whisper-large-v3".to_string(),
    ///     language: Some("en".to_string()),
    ///     prompt: Some("This is a business meeting transcript.".to_string()),
    ///     response_format: Some("json".to_string()),
    ///     temperature: Some(0.0),
    ///     timestamp_granularities: Some(vec!["word".to_string()]),
    /// };
    /// 
    /// let result = client.audio().transcribe(request).await?;
    /// println!("Transcribed text: {}", result.text);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn transcribe(self, req: AudioTranscriptionRequest) -> Result<Transcription, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_multipart("audio/transcriptions", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Translates audio to English text
    /// 
    /// # Arguments
    /// 
    /// * `req` - The translation request parameters
    /// 
    /// # Returns
    /// 
    /// A `Translation` containing the translated text and metadata
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the translation fails
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{GroqClientBuilder, AudioTranslationRequest};
    /// use std::path::PathBuf;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let request = AudioTranslationRequest {
    ///     file: Some(PathBuf::from("spanish_interview.mp3")),
    ///     url: None,
    ///     model: "whisper-large-v3".to_string(),
    ///     prompt: Some("This is an interview transcript.".to_string()),
    ///     response_format: Some("json".to_string()),
    ///     temperature: Some(0.0),
    /// };
    /// 
    /// let result = client.audio().translate(request).await?;
    /// println!("Translated text: {}", result.text);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn translate(self, req: AudioTranslationRequest) -> Result<Translation, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_multipart("audio/translations", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}

impl Default for AudioTranscriptionRequest {
    fn default() -> Self {
        Self {
            file: None,
            url: None,
            model: String::new(),
            language: None,
            prompt: None,
            response_format: None,
            temperature: None,
            timestamp_granularities: None,
        }
    }
}

impl Default for AudioTranslationRequest {
    fn default() -> Self {
        Self {
            file: None,
            url: None,
            model: String::new(),
            prompt: None,
            response_format: None,
            temperature: None,
        }
    }
}