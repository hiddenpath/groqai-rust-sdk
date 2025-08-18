use groqai::GroqClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Method 1: Using environment variables (recommended for production)
    println!("=== Using Environment Variables ===");
    std::env::set_var("GROQ_API_KEY", "gsk_your_api_key_here");
    
    match GroqClient::new() {
        Ok(_client) => println!("✓ Client created successfully from environment"),
        Err(e) => println!("✗ Failed to create client: {}", e),
    }
    
    // Method 2: Direct API key
    println!("\n=== Using Direct API Key ===");
    match GroqClient::with_api_key("gsk_your_api_key_here") {
        Ok(_client) => println!("✓ Client created successfully with API key"),
        Err(e) => println!("✗ Failed to create client: {}", e),
    }
    
    // Method 3: Environment variables with proxy and timeout
    println!("\n=== Using Environment Variables with Proxy ===");
    std::env::set_var("GROQ_PROXY_URL", "http://proxy.example.com:8080");
    std::env::set_var("GROQ_TIMEOUT_SECS", "60");
    
    match GroqClient::from_env() {
        Ok(_client) => println!("✓ Client created with proxy configuration"),
        Err(e) => println!("✗ Failed to create client: {}", e),
    }
    
    println!("\n=== Environment Variables Reference ===");
    println!("Required:");
    println!("  GROQ_API_KEY=gsk_your_api_key_here");
    println!("\nOptional:");
    println!("  GROQ_PROXY_URL=http://proxy.example.com:8080");
    println!("  GROQ_TIMEOUT_SECS=60");
    
    Ok(())
}