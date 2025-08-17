# GroqAI - Rust Client SDK Library

[![Crates.io](https://img.shields.io/crates/v/groqai.svg)](https://crates.io/crates/groqai)
[![Documentation](https://docs.rs/groqai/badge.svg)](https://docs.rs/groqai)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

A modern, type-safe Rust SDK for the [Groq API](https://groq.com/) with enterprise-grade features, providing lightning-fast AI inference capabilities with comprehensive error handling and built-in resilience.

## Features

- 🚀 **High Performance** - Built for speed with async/await support and efficient HTTP transport
- 💬 **Chat Completions** - Support for both streaming and non-streaming conversations with advanced message types
- 🎵 **Audio Processing** - Transcription and translation using Whisper models with file and URL support
- 📁 **File Management** - Complete file lifecycle management (upload, list, retrieve, delete)
- 🔄 **Batch Processing** - Efficient bulk operations for large-scale tasks with status monitoring
- 🤖 **Model Information** - Retrieve available models and their detailed capabilities
- 🎯 **Fine-tuning** - Custom model training support with supervised learning
- 🛡️ **Enterprise Error Handling** - Comprehensive error types, automatic retries, and graceful degradation
- 📊 **Smart Rate Limiting** - Built-in rate limiting with exponential backoff and retry-after header support
- 🔧 **Flexible Configuration** - Customizable timeouts, proxies, base URLs, and transport settings
- 🔒 **Type Safety** - Strongly typed API with compile-time guarantees
- 🌐 **Proxy Support** - Full HTTP/HTTPS proxy support for enterprise environments
- 📝 **Rich Message Types** - Support for text, images, and multi-part messages
- 🔄 **Conversation Management** - Built-in conversation history management with token optimization

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
groqai = "0.1.9"
tokio = { version = "1.47", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

Or install via cargo:

```bash
cargo add groqai
cargo add tokio --features full
```

### Basic Usage

```rust
use groqai::{GroqClientBuilder, ChatMessage, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = GroqClientBuilder::new("gsk_your_api_key_here".to_string())?
        .build()?;
    
    // Create a chat completion
    let messages = vec![
        ChatMessage::new_text(Role::User, "Explain quantum computing in simple terms")
    ];
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .messages(messages)
        .temperature(0.7)
        .max_completion_tokens(500)
        .send()
        .await?;
    
    println!("Response: {}", response.choices[0].message.content);
    Ok(())
}
```

## API Reference

### Chat Completions

#### Non-streaming Chat

```rust
use groqai::{GroqClientBuilder, ChatMessage, Role};

let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;

let messages = vec![
    ChatMessage::new_text(Role::System, "You are a helpful assistant."),
    ChatMessage::new_text(Role::User, "What is the capital of France?"),
];

let response = client
    .chat("llama-3.1-70b-versatile")
    .messages(messages)
    .temperature(0.7)
    .send()
    .await?;

println!("{}", response.choices[0].message.content);
```

#### Streaming Chat

```rust
use futures::StreamExt;

let mut stream = client
    .chat("llama-3.1-70b-versatile")
    .messages(messages)
    .stream(true)
    .send_stream()
    .await?;

while let Some(chunk) = stream.next().await {
    match chunk {
        Ok(chunk) => {
            if let Some(content) = &chunk.choices[0].delta.content {
                print!("{}", content);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Audio Processing

#### Transcription

```rust
use groqai::AudioTranscriptionRequest;
use std::path::PathBuf;

let request = AudioTranscriptionRequest {
    file: Some(PathBuf::from("audio.mp3")),
    url: None,
    model: "whisper-large-v3".to_string(),
    language: Some("en".to_string()),
    prompt: None,
    response_format: Some("json".to_string()),
    temperature: Some(0.0),
    timestamp_granularities: None,
};

let transcription = client.audio().transcribe(request).await?;
println!("Transcription: {}", transcription.text);
```

#### Translation

```rust
use groqai::AudioTranslationRequest;

let request = AudioTranslationRequest {
    file: Some(PathBuf::from("spanish_audio.mp3")),
    url: None,
    model: "whisper-large-v3".to_string(),
    prompt: None,
    response_format: Some("json".to_string()),
    temperature: Some(0.0),
};

let translation = client.audio().translate(request).await?;
println!("Translation: {}", translation.text);
```

### File Management

```rust
use groqai::FileCreateRequest;
use std::path::PathBuf;

// Upload a file
let request = FileCreateRequest::new(
    PathBuf::from("training_data.jsonl"),
    "batch".to_string()
)?;
let file = client.files().create(request).await?;

// List files
let files = client.files().list().await?;
for file in files.data {
    println!("File: {} ({})", file.filename, file.purpose);
}

// Retrieve a file
let file = client.files().retrieve("file_id".to_string()).await?;

// Delete a file
let deletion = client.files().delete("file_id".to_string()).await?;
```

### Batch Processing

```rust
use groqai::BatchCreateRequest;

// Create a batch job
let request = BatchCreateRequest {
    input_file_id: "file_abc123".to_string(),
    endpoint: "/chat/completions".to_string(),
    completion_window: "24h".to_string(),
    metadata: None,
};

let batch = client.batches().create(request).await?;
println!("Batch created: {}", batch.id);

// Check batch status
let batch = client.batches().retrieve(batch.id).await?;
println!("Status: {}", batch.status);

// List batches
let batches = client.batches().list(None, Some(10)).await?;
```

### Model Information

```rust
// List available models
let models = client.models().list().await?;
for model in models.data {
    println!("Model: {} - {}", model.id, model.owned_by);
}

// Get model details
let model = client.models().retrieve("llama-3.1-70b-versatile".to_string()).await?;
println!("Context window: {} tokens", model.context_window);
```

## Configuration

### Custom Configuration

```rust
use std::time::Duration;
use url::Url;

let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    .base_url(Url::parse("https://api.groq.com/openai/v1/")?)
    .timeout(Duration::from_secs(60))
    .build()?;
```

### Using Proxy

```rust
let proxy = reqwest::Proxy::http("http://proxy.example.com:8080")?;
let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?
    .proxy(proxy)
    .build()?;
```

## Error Handling

The library provides comprehensive error handling through the `GroqError` enum:

```rust
use groqai::GroqError;

match client.chat("model").messages(messages).send().await {
    Ok(response) => println!("Success: {}", response.choices[0].message.content),
    Err(GroqError::RateLimited) => println!("Rate limited, please retry later"),
    Err(GroqError::InvalidApiKey(_)) => println!("Invalid API key"),
    Err(GroqError::Api(api_error)) => println!("API error: {}", api_error.error.message),
    Err(e) => println!("Other error: {}", e),
}
```

## Supported Models

The SDK supports all current Groq models with built-in type safety:

### Chat Models
- **Llama 3.1 Series**: 
  - `llama-3.1-8b-instant` - Fast responses for simple tasks
  - `llama-3.1-70b-versatile` - Balanced performance and capability
  - `llama-3.1-405b-reasoning` - Advanced reasoning and complex tasks
  - `llama-3.3-70b-versatile` - Latest model with enhanced capabilities
- **Mixtral**: `mixtral-8x7b-32768` - Large context window for complex conversations
- **Gemma**: `gemma2-9b-it` - Efficient instruction-tuned model
- **Qwen**: `qwen2.5-72b-instruct` - Multilingual capabilities

### Audio Models
- **Whisper**: `whisper-large-v3` - State-of-the-art speech recognition and translation

### Model Selection Helper
```rust
use groqai::KnownModel;

// Type-safe model selection
let model = KnownModel::Llama3_1_70bVersatile;
let response = client.chat(&model.to_string()).send().await?;
```

## Rate Limiting

The client includes built-in rate limiting with exponential backoff:

```rust
// Rate limiting is handled automatically
let response = client.chat("model").messages(messages).send().await?;
```

## Advanced Features

### Multi-Modal Messages
```rust
use groqai::{ChatMessage, Role, MessageContent, ImageUrl};

let messages = vec![
    ChatMessage {
        role: Role::User,
        content: MessageContent::Parts(vec![
            MessagePart::Text { text: "What's in this image?".to_string() },
            MessagePart::ImageUrl { 
                image_url: ImageUrl::new("https://example.com/image.jpg") 
            },
        ]),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }
];
```

### Conversation History Management
```rust
// Built-in conversation management with token optimization
let mut conversation = Vec::new();
conversation.push(ChatMessage::new_text(Role::User, "Hello"));

// Automatic history trimming to stay within token limits
trim_conversation_history(&mut conversation, 15, 18000);
```

### Enterprise Proxy Configuration
```rust
use reqwest::Proxy;

let proxy = Proxy::all("http://corporate-proxy:8080")?
    .basic_auth("username", "password");

let client = GroqClientBuilder::new(api_key)?
    .proxy(proxy)
    .timeout(Duration::from_secs(120))
    .build()?;
```

## Examples

Check out the `examples/` directory for comprehensive examples:

- `cli_chat.rs` - Interactive CLI chat application with streaming support
- `chat_completion.rs` - Basic chat completion
- `streaming_chat.rs` - Streaming responses
- `audio_transcription.rs` - Audio processing
- `batch_processing.rs` - Batch operations
- `file_management.rs` - File operations
- `model_info.rs` - Model information and capabilities

## Requirements

- Rust 1.70 or later
- A valid Groq API key (get one at [console.groq.com](https://console.groq.com/))

## Project Status

This SDK is actively maintained and production-ready. Current version: **0.1.9**

### Roadmap

- ✅ Chat Completions (streaming & non-streaming)
- ✅ Audio Transcription & Translation
- ✅ File Management
- ✅ Batch Processing
- ✅ Model Information
- ✅ Fine-tuning Support
- ✅ Enterprise Features (proxy, rate limiting)
- 🔄 Function Calling (in progress)
- 📋 Vision API enhancements
- 📋 Advanced streaming features

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Groq](https://groq.com/) for providing the lightning-fast AI inference API
- The Rust community for excellent async and HTTP libraries
- Contributors and users who help improve this SDK

## Architecture

The SDK is built with a modular architecture:

- **Transport Layer** (`transport.rs`) - HTTP client with retry logic and rate limiting
- **API Modules** (`api/`) - Endpoint-specific implementations for each Groq service
- **Type System** (`types.rs`) - Strongly typed request/response structures
- **Error Handling** (`error.rs`) - Comprehensive error types with context
- **Rate Limiting** (`rate_limit.rs`) - Smart rate limiting with exponential backoff
- **Client Builder** (`client.rs`) - Flexible client configuration

## Performance Considerations

- **Async/Await**: Built on Tokio for high-performance async operations
- **Connection Pooling**: Reuses HTTP connections for better performance
- **Streaming**: Efficient streaming for real-time applications
- **Memory Management**: Optimized for low memory footprint
- **Rate Limiting**: Prevents API quota exhaustion with smart backoff

## Security Features

- **API Key Validation**: Validates API key format at build time
- **HTTPS Only**: All communications use TLS encryption
- **Proxy Support**: Full support for corporate proxy environments
- **Error Sanitization**: Sensitive data is not logged in error messages

## Testing

The SDK includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test tests::chat
cargo test tests::audio
cargo test tests::files
```

## Support

- 📖 [Documentation](https://docs.rs/groqai)
- 🐛 [Issue Tracker](https://github.com/hiddenpath/groqai-rust-sdk/issues)
- 💬 [Discussions](https://github.com/hiddenpath/groqai-rust-sdk/discussions)
- 📧 [Author](mailto:alex.wang@msn.com)

---

**Note**: This is an unofficial client SDK. For official support, please refer to the [Groq documentation](https://console.groq.com/docs).