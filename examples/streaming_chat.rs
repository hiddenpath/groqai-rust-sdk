// examples/streaming_chat.rs
// Streaming chat completion example

use groqai::{GroqClient, ChatMessage, Role};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using environment variables (recommended)
    let client = GroqClient::new()?;
    
    let messages = vec![
        ChatMessage::new_text(Role::User, "Write a short story about a robot learning to paint"),
    ];
    
    let mut stream = client
        .chat("llama-3.1-70b-versatile")
        .messages(messages)
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