// examples/chat_completion.rs
// Basic chat completion example

use groqai::{GroqClient, ChatMessage, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using environment variables (recommended)
    let client = GroqClient::new()?;
    
    let messages = vec![
        ChatMessage::new_text(Role::System, "You are a helpful assistant."),
        ChatMessage::new_text(Role::User, "Explain quantum computing in simple terms"),
    ];
    
    let response = client
        .chat("llama-3.1-70b-versatile")
        .messages(messages)
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