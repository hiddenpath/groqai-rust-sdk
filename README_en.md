# Groq AI Rust SDK

[![Crates.io](https://img.shields.io/crates/v/groqai)](https://crates.io/crates/groqai)
[![Documentation](https://docs.rs/groqai/badge.svg)](https://docs.rs/groqai)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A modern, type-safe Rust SDK for interacting with the Groq AI API.

## ✨ Features

- 🔒 **Type Safety** - Complete Rust type system support
- 🚀 **Async Support** - High-performance async API
- 📊 **Streaming** - Real-time streaming chat completions
- 🎵 **Audio Processing** - Audio transcription, translation, and speech synthesis
- 📁 **File Management** - Complete file upload, download, and management
- 🔧 **Tool Calling** - Native support for function calls and tools
- 🖼️ **Multimodal** - Support for text and image input
- 📦 **Batch Processing** - Efficient batch job processing
- 🏗️ **Builder Pattern** - Clear parameter configuration

## 📦 Installation

```toml
[dependencies]
groqai = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 Quick Start

### Basic Chat

```rust
use groqai::{GroqClient, ChatMessage, Role, ChatCompletionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::new("your-api-key".to_string());
    
    let message = ChatMessage::new_text(
        Role::User,
        "Hello, please introduce yourself".to_string()
    );
    
    let request = ChatCompletionRequest {
        messages: vec![message],
        model: "llama3-8b-8192".to_string(),
        temperature: Some(0.7),
        stream: Some(false),
        ..Default::default()
    };
    
    let response = client.chat_completions(request).await?;
    println!("Response: {}", response.choices[0].message.content);
    Ok(())
}
```

### Streaming Chat

```rust
use groqai::{GroqClient, ChatMessage, Role, ChatCompletionRequest};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::new("your-api-key".to_string());
    
    let message = ChatMessage::new_text(
        Role::User,
        "Write a short poem about Rust".to_string()
    );
    
    let request = ChatCompletionRequest {
        messages: vec![message],
        model: "llama3-8b-8192".to_string(),
        stream: Some(true),
        ..Default::default()
    };
    
    let mut stream = client.stream_chat_completions(request).await?;
    
    while let Some(chunk) = stream.try_next().await? {
        if let Some(delta) = &chunk.choices[0].delta {
            print!("{}", delta.content);
        }
    }
    println!();
    Ok(())
}
```

### Audio Transcription

```rust
use groqai::{GroqClient, AudioTranscriptionRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::new("your-api-key".to_string());
    
    let request = AudioTranscriptionRequest::new(
        "base64_encoded_audio_content".to_string(),
        "whisper-large-v3".to_string(),
    )
    .language("en".to_string())
    .prompt("This is an English audio clip".to_string());
    
    let transcription = client.audio_transcription(request, "audio.mp3").await?;
    println!("Transcription: {}", transcription.text);
    Ok(())
}
```

### File Management

```rust
use groqai::GroqClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClient::new("your-api-key".to_string());
    
    // Upload file
    let file = client.upload_file("data.json", "batch").await?;
    println!("File ID: {}", file.id);
    
    // List files
    let files = client.list_files().await?;
    for file in &files.data {
        println!("File: {} ({} bytes)", file.filename, file.bytes);
    }
    
    // Delete file
    let delete_result = client.delete_file(&file.id).await?;
    println!("Delete successful: {}", delete_result.deleted);
    
    Ok(())
}
```

## 📚 API Reference

### Core Types

- `GroqClient` - Main API client
- `ChatMessage` - Chat message structure
- `Role` - Message role enum
- `MessageContent` - Message content type
- `ToolCall` - Tool call structure

### Main Methods

#### Chat Completions
- `chat_completions()` - Chat completion API
- `stream_chat_completions()` - Streaming chat completion

#### Audio Processing
- `audio_transcription()` - Audio transcription
- `audio_translation()` - Audio translation
- `audio_speech()` - Speech synthesis

#### File Management
- `upload_file()` - Upload file
- `list_files()` - List files
- `retrieve_file()` - Get file information
- `download_file()` - Download file
- `delete_file()` - Delete file

#### Batch Processing
- `create_batch()` - Create batch job
- `retrieve_batch()` - Query batch job
- `list_batches()` - List batch jobs
- `cancel_batch()` - Cancel batch job

## 🛠️ Error Handling

```rust
use groqai::{GroqClient, GroqError};

match client.chat_completions(request).await {
    Ok(response) => {
        println!("Success: {}", response.choices[0].message.content);
    }
    Err(GroqError::Api(status, message)) => {
        eprintln!("API Error: {} - {}", status, message);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## 📖 More Examples

Check the `examples/` directory for complete examples:

```bash
cargo run --example modern_examples
```

## 🤝 Contributing

Welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

- 💬 Discussions: [GitHub Discussions](https://github.com/hiddenpath/groqai-rust-sdk/discussions)
- 🐛 Issues: [GitHub Issues](https://github.com/hiddenpath/groqai-rust-sdk/issues)
- 📖 Documentation: [docs.rs/groqai](https://docs.rs/groqai)
- 🔗 Official API Documentation: [Groq API Reference](https://console.groq.com/docs/api-reference)

---

**Note**: This is an unofficial Groq AI SDK. Groq AI does not provide official support for this SDK.
