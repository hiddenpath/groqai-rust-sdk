// examples/model_info.rs
// Model information example

use groqai::GroqClientBuilder;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    
    let client = GroqClientBuilder::new(api_key)?.build()?;
    
    // List available models
    match client.models().list().await {
        Ok(models) => {
            println!("Available models:");
            for model in models.data {
                println!("- {}: {} tokens (owned by {})", 
                    model.id, 
                    model.context_window,
                    model.owned_by
                );
            }
        }
        Err(e) => println!("Failed to list models: {}", e),
    }
    
    // Get specific model details
    match client.models().retrieve("llama-3.1-70b-versatile".to_string()).await {
        Ok(model) => {
            println!("\nModel details for {}:", model.id);
            println!("Context window: {} tokens", model.context_window);
            println!("Owned by: {}", model.owned_by);
        }
        Err(e) => println!("Failed to retrieve model details: {}", e),
    }
    
    Ok(())
}