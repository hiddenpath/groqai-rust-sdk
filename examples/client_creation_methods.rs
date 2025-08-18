// examples/client_creation_methods.rs
// Comprehensive example showing all client creation methods

use groqai::{GroqClient, GroqClientBuilder, ChatMessage, Role};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GroqClient Creation Methods ===\n");

    // Method 1: Environment Variables (Recommended for Production)
    println!("1. Using Environment Variables (GroqClient::new()):");
    std::env::set_var("GROQ_API_KEY", "gsk_your_api_key_here");
    
    match GroqClient::new() {
        Ok(client) => {
            println!("   ✓ Client created successfully");
            demo_chat(&client, "Environment variables").await?;
        }
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    // Method 2: Direct API Key (Quick Setup)
    println!("\n2. Using Direct API Key (GroqClient::with_api_key()):");
    match GroqClient::with_api_key("gsk_your_api_key_here") {
        Ok(client) => {
            println!("   ✓ Client created successfully");
            demo_chat(&client, "Direct API key").await?;
        }
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    // Method 3: Builder Pattern (Advanced Configuration)
    println!("\n3. Using Builder Pattern (GroqClientBuilder):");
    match GroqClientBuilder::new("gsk_your_api_key_here".to_string())?
        .timeout(Duration::from_secs(60))
        .build()
    {
        Ok(client) => {
            println!("   ✓ Client created with custom timeout");
            demo_chat(&client, "Builder pattern").await?;
        }
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    // Method 4: Environment Variables with Proxy
    println!("\n4. Using Environment Variables with Proxy:");
    std::env::set_var("GROQ_PROXY_URL", "http://proxy.example.com:8080");
    std::env::set_var("GROQ_TIMEOUT_SECS", "45");
    
    match GroqClient::from_env() {
        Ok(client) => {
            println!("   ✓ Client created with proxy configuration");
            demo_chat(&client, "Environment with proxy").await?;
        }
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    println!("\n=== Summary ===");
    println!("• GroqClient::new() - Simplest, uses environment variables");
    println!("• GroqClient::with_api_key() - Quick setup with direct API key");
    println!("• GroqClient::from_env() - Explicit environment variable usage");
    println!("• GroqClientBuilder - Full control over all settings");

    Ok(())
}

async fn demo_chat(_client: &GroqClient, method: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _messages = vec![
        ChatMessage::new_text(Role::User, "Say hello in one word")
    ];

    // Note: This would fail with fake API key, but demonstrates the API
    println!("   → Testing chat with {}: Ready to send request", method);
    Ok(())
}