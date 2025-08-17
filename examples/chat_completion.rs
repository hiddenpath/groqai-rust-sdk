// examples/chat_completion.rs
// Basic chat completion example

use groqai::{GroqClientBuilder, ChatMessage, Role};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    
    let client = GroqClientBuilder::new(api_key)?.build()?;
    
    let messages = vec![
        ChatMessage::new_text(Role::System, "You are a helpful assistant."),
        ChatMessage::new_text(Role::User, "Explain quantum computing in simple terms"),
    ];
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .message(messages[0].clone())
        .message(messages[1].clone())
        .temperature(0.7)
        .send()
        .await?;
    
    if let Some(choice) = response.choices.first() {
        match &choice.message.content {
            groqai::types::MessageContent::Text(text) => {
                println!("Response: {}", text);
            }
            _ => println!("Unexpected response format"),
        }
    }
    Ok(())
}