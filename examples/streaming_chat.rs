// examples/streaming_chat.rs
// Streaming chat completion example

use groqai::{GroqClientBuilder, ChatMessage, Role};
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    
    let client = GroqClientBuilder::new(api_key)?.build()?;
    
    let messages = vec![
        ChatMessage::new_text(Role::User, "Write a short story about a robot learning to paint"),
    ];
    
    let mut stream = client
        .chat("llama-3.1-70b-versatile")
        .message(messages[0].clone())
        .temperature(0.8)
        .stream(true)
        .send_stream()
        .await?;
    
    print!("Response: ");
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                if let Some(choice) = chunk.choices.first() {
                    if let Some(content) = &choice.delta.content {
                        match content {
                            groqai::types::MessageContent::Text(text) => {
                                print!("{}", text);
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => eprintln!("Stream error: {}", e),
        }
    }
    println!();
    
    Ok(())
}