use groqai::client::GroqClientBuilder;
use groqai::error::GroqError;
use groqai::types::{ChatMessage, Role, Tool, FunctionDef, ToolChoice};
use futures_util::stream::StreamExt;
use std::env;

fn create_client() -> Result<GroqClientBuilder, GroqError> {
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    let mut builder = GroqClientBuilder::new(api_key)?;
    
    // 添加代理支持
    if let Ok(proxy_url) = env::var("PROXY_URL") {
        let proxy = reqwest::Proxy::all(&proxy_url).map_err(|e| {
            GroqError::InvalidMessage(format!("Invalid proxy URL: {}", e))
        })?;
        builder = builder.proxy(proxy);
    }
    
    Ok(builder)
}

#[tokio::test]
async fn test_chat_non_streaming() -> Result<(), GroqError> {
    let client = create_client()?.build()?;
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .message(ChatMessage::new_text(Role::User, "Hello, how are you?"))
        .temperature(0.7)
        .frequency_penalty(0.5)
        .presence_penalty(0.3)
        .send()
        .await?;
    
    assert!(response.choices.first().is_some());
    assert_eq!(response.object, "chat.completion");
    Ok(())
}

#[tokio::test]
async fn test_chat_streaming() -> Result<(), GroqError> {
    let client = create_client()?.build()?;
    
    let mut stream = client
        .chat("llama-3.1-70b-versatile")
        .message(ChatMessage::new_text(Role::User, "Tell me a short story"))
        .stream(true)
        .send_stream()
        .await?;
    
    let first_chunk = stream.next().await.unwrap()?;
    assert_eq!(first_chunk.object, "chat.completion.chunk");
    Ok(())
}

#[tokio::test]
async fn test_chat_with_tools() -> Result<(), GroqError> {
    let client = create_client()?.build()?;
    
    let tools = vec![Tool {
        type_: "function".to_string(),
        function: FunctionDef {
            name: "get_weather".to_string(),
            description: Some("Get current weather".to_string()),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "location": { "type": "string" }
                },
                "required": ["location"]
            }),
        },
    }];
    
    let tool_choice = ToolChoice {
        type_: "function".to_string(),
        function: Some(serde_json::json!({ "name": "get_weather" })),
    };
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .message(ChatMessage::new_text(Role::User, "What's the weather in Tokyo?"))
        .tools(tools)
        .tool_choice(tool_choice)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Tool test error: {:?}", e);
            e
        })?;
    
    assert!(response.choices.first().is_some());
    Ok(())
}

#[tokio::test]
async fn test_chat_with_logprobs() -> Result<(), GroqError> {
    let client = create_client()?.build()?;
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .message(ChatMessage::new_text(Role::User, "Test logprobs"))
        .logprobs(false)
        .top_logprobs(5)
        .send()
        .await?;
    
    assert!(response.choices.first().is_some());
    Ok(())
}