// examples/environment_setup.rs
// Demonstrates environment variable requirements and error handling

use groqai::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GroqAI Environment Setup Demo ===\n");

    // ========================================================================
    // Scenario 1: Missing GROQ_API_KEY
    // ========================================================================
    println!("1. Testing without GROQ_API_KEY environment variable:");
    
    // Remove the environment variable to demonstrate error
    std::env::remove_var("GROQ_API_KEY");
    
    match GroqClient::new() {
        Ok(_) => println!("   ✗ Unexpected success"),
        Err(GroqError::InvalidApiKey(msg)) => {
            println!("   ✓ Expected error: {}", msg);
            println!("   → Solution: Set GROQ_API_KEY environment variable");
        }
        Err(e) => println!("   ✗ Unexpected error: {}", e),
    }

    // ========================================================================
    // Scenario 2: Setting GROQ_API_KEY
    // ========================================================================
    println!("\n2. Setting GROQ_API_KEY environment variable:");
    
    std::env::set_var("GROQ_API_KEY", "gsk_demo_key_12345");
    println!("   $ export GROQ_API_KEY=\"gsk_your_api_key_here\"");
    
    match GroqClient::new() {
        Ok(_client) => {
            println!("   ✓ Client created successfully!");
            println!("   → Ready to make API calls");
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // ========================================================================
    // Scenario 3: Optional environment variables
    // ========================================================================
    println!("\n3. Optional environment variables:");
    
    println!("   Setting optional proxy configuration:");
    std::env::set_var("GROQ_PROXY_URL", "http://proxy.example.com:8080");
    std::env::set_var("GROQ_TIMEOUT_SECS", "45");
    
    println!("   $ export GROQ_PROXY_URL=\"http://proxy.example.com:8080\"");
    println!("   $ export GROQ_TIMEOUT_SECS=\"45\"");
    
    match GroqClient::from_env() {
        Ok(_client) => {
            println!("   ✓ Client created with proxy and custom timeout!");
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // ========================================================================
    // Summary
    // ========================================================================
    println!("\n=== Environment Variables Summary ===");
    println!("Required:");
    println!("  GROQ_API_KEY     - Your Groq API key (starts with 'gsk_')");
    println!("\nOptional:");
    println!("  GROQ_PROXY_URL   - HTTP/HTTPS proxy URL");
    println!("  GROQ_TIMEOUT_SECS - Request timeout in seconds (default: 30)");
    println!("  HTTPS_PROXY      - Alternative proxy setting");
    println!("  HTTP_PROXY       - Alternative proxy setting");
    
    println!("\n=== Quick Setup Commands ===");
    println!("# Get your API key from: https://console.groq.com/");
    println!("export GROQ_API_KEY=\"gsk_your_actual_api_key_here\"");
    println!("");
    println!("# Optional: Set proxy if needed");
    println!("export GROQ_PROXY_URL=\"http://your-proxy:8080\"");
    println!("");
    println!("# Then use in your code:");
    println!("use groqai::prelude::*;");
    println!("let client = GroqClient::new()?;");

    Ok(())
}