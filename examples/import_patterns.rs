// examples/import_patterns.rs
// Demonstrates different import patterns for the groqai library

// ============================================================================
// Pattern 1: Prelude Import (Recommended for beginners)
// ============================================================================
use groqai::prelude::*;

// ============================================================================
// Pattern 2: Specific Imports (Recommended for libraries)
// ============================================================================
// use groqai::{GroqClient, ChatMessage, Role, GroqError};

// ============================================================================
// Pattern 3: Granular Imports (For advanced users)
// ============================================================================
// use groqai::{
//     GroqClient, GroqClientBuilder,
//     ChatMessage, Role, MessageContent, 
//     ChatCompletionResponse, Choice,
//     GroqError,
// };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GroqAI Import Patterns Demo ===\n");

    // Set up environment for demo
    std::env::set_var("GROQ_API_KEY", "gsk_demo_key_12345");

    // ========================================================================
    // Using Prelude Import - Most Convenient
    // ========================================================================
    println!("1. Using prelude import (use groqai::prelude::*):");
    
    match GroqClient::new() {
        Ok(_client) => {
            println!("   ✓ Client created successfully");
            
            // All common types are available without qualification
            let _messages = vec![
                ChatMessage::new_text(Role::User, "Hello")
            ];
            
            println!("   ✓ ChatMessage and Role available directly");
            println!("   ✓ Perfect for: Applications, quick prototypes, learning");
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // ========================================================================
    // Demonstrating Available Types
    // ========================================================================
    println!("\n2. Available types with prelude import:");
    
    // Core client types
    println!("   • GroqClient - Main client");
    println!("   • GroqError - Error handling");
    
    // Message types
    println!("   • ChatMessage - Chat messages");
    println!("   • Role - Message roles (User, Assistant, System, Tool)");
    println!("   • MessageContent - Message content types");
    
    // Model types
    println!("   • KnownModel - Type-safe model selection");
    
    // Response types
    println!("   • ChatCompletionResponse - API responses");

    // ========================================================================
    // Best Practices
    // ========================================================================
    println!("\n3. Import Pattern Recommendations:");
    println!("   📚 Learning/Prototyping: use groqai::prelude::*");
    println!("   🏢 Applications: use groqai::{{GroqClient, ChatMessage, Role}}");
    println!("   📦 Libraries: Specific imports to avoid conflicts");
    println!("   🔧 Advanced: Granular imports for precise control");

    // ========================================================================
    // Error Handling Example
    // ========================================================================
    println!("\n4. Error handling with prelude:");
    
    match GroqClient::with_api_key("invalid_key") {
        Ok(_) => println!("   Unexpected success"),
        Err(GroqError::InvalidApiKey(msg)) => {
            println!("   ✓ Caught InvalidApiKey: {}", msg);
        }
        Err(e) => println!("   ✗ Other error: {}", e),
    }

    println!("\n=== Demo Complete ===");
    println!("Choose the import pattern that best fits your use case!");

    Ok(())
}