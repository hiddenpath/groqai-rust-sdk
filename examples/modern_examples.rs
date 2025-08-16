use groqai::{
    ChatMessage, Role, MessageContent, MessagePart, ImageUrl,
    AudioTranscriptionRequest, AudioTranslationRequest, ChatCompletionRequest,
    Tool, FunctionDef, ToolCall, FunctionCall,
    GroqClient, GroqClientBuilder
};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GroqClientBuilder::new("your-api-key".to_string())?.build()?;

    // Example 1: Basic text message
    println!("=== Example 1: Basic Text Message ===");
    let simple_message = ChatMessage::new_text(
        Role::User,
        "Hello, please introduce yourself".to_string()
    );

    // Example 2: Multimodal message (text + image)
    println!("\n=== Example 2: Multimodal Message ===");
    let multimodal_message = ChatMessage::new_multimodal(
        Role::User,
        vec![
            MessagePart::Text {
                text: "Please describe this image".to_string(),
            },
            MessagePart::ImageUrl {
                image_url: ImageUrl {
                    url: "https://example.com/image.jpg".to_string(),
                    detail: Some("high".to_string()),
                },
            },
        ]
    );

    // Example 3: Function calling setup
    println!("\n=== Example 3: Function Calling ===");
    let function_def = FunctionDef {
        name: "get_weather".to_string(),
        description: Some("Get weather information for a specific city".to_string()),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "City name"
                }
            },
            "required": ["city"]
        }),
    };

    let tool = Tool {
        type_: "function".to_string(),
        function: function_def,
    };

    let function_message = ChatMessage::new_text(
        Role::User,
        "What's the weather like in Beijing today?".to_string()
    );

    // Example 4: Audio transcription request
    println!("\n=== Example 4: Audio Transcription Request ===");
    let transcription_request = AudioTranscriptionRequest {
        file: None,
        url: Some("https://example.com/audio.mp3".to_string()),
        model: "whisper-large-v3".to_string(),
        language: Some("en".to_string()),
        prompt: Some("This is an English audio clip".to_string()),
        response_format: Some("text".to_string()),
        temperature: Some(0.0),
        timestamp_granularities: None,
    };

    // Example 5: Audio translation request
    println!("\n=== Example 5: Audio Translation Request ===");
    let translation_request = AudioTranslationRequest {
        file: None,
        url: Some("https://example.com/audio.mp3".to_string()),
        model: "whisper-large-v3".to_string(),
        prompt: Some("This is an audio clip that needs translation".to_string()),
        response_format: Some("text".to_string()),
        temperature: Some(0.0),
    };

    // Example 6: Tool response message
    println!("\n=== Example 6: Tool Response ===");
    let tool_response = ChatMessage::tool_response(
        "call_123".to_string(),
        "Beijing is sunny today with a temperature of 25°C".to_string()
    );

    // Example 7: Message with tool calls
    println!("\n=== Example 7: Message with Tool Calls ===");
    let tool_call = ToolCall {
        id: "call_123".to_string(),
        type_: "function".to_string(),
        function: FunctionCall {
            name: "get_weather".to_string(),
            arguments: r#"{"city": "Beijing"}"#.to_string(),
        },
    };

    let message_with_tool_calls = ChatMessage {
        role: Role::Assistant,
        content: MessageContent::Text("I'll check the weather for you.".to_string()),
        tool_calls: Some(vec![tool_call]),
        tool_call_id: None,
    };

    // Example 8: Chat completion request
    println!("\n=== Example 8: Chat Completion Request ===");
    let chat_request = ChatCompletionRequest {
        messages: vec![simple_message],
        model: "llama-3.1-70b-versatile".to_string(),
        temperature: Some(0.7),
        max_completion_tokens: Some(1000),
        stream: Some(false),
        tools: Some(vec![tool]),
        ..Default::default()
    };

    // Example 9: Streaming chat completion
    println!("\n=== Example 9: Streaming Chat Completion ===");
    let streaming_request = ChatCompletionRequest {
        messages: vec![ChatMessage::new_text(
            Role::User,
            "Write a short poem about Rust".to_string()
        )],
        model: "llama-3.1-70b-versatile".to_string(),
        stream: Some(true),
        temperature: Some(0.8),
        ..Default::default()
    };

    // Example 10: File operations
    println!("\n=== Example 10: File Operations ===");
    // Note: These are examples of how to use the API, not actual calls
    println!("Upload file: client.upload_file(\"data.json\", \"batch\").await?");
    println!("List files: client.list_files().await?");
    println!("Delete file: client.delete_file(&file_id).await?");

    // Example 11: Batch operations
    println!("\n=== Example 11: Batch Operations ===");
    println!("Create batch: client.create_batch(\"file_id\", \"24h\").await?");
    println!("Retrieve batch: client.retrieve_batch(&batch_id).await?");
    println!("List batches: client.list_batches().await?");

    // Example 12: Audio speech synthesis
    println!("\n=== Example 12: Audio Speech Synthesis ===");
    println!("Speech synthesis: client.audio_speech(\"tts-1\", \"Hello world\", \"alloy\", None, None, None).await?");

    println!("\nAll example structures created successfully!");
    println!("These improvements provide better type safety and clearer API design.");

    Ok(())
}
