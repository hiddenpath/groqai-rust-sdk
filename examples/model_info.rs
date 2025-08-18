// examples/model_info.rs
// Model information example

use groqai::GroqClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using environment variables (recommended)
    let client = GroqClient::new()?;
    
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