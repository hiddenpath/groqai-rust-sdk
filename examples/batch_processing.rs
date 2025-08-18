// examples/batch_processing.rs
// Batch processing example

use groqai::{GroqClient, BatchCreateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using environment variables (recommended)
    let client = GroqClient::new()?;
    
    // Create a batch job
    let request = BatchCreateRequest {
        input_file_id: "file_abc123".to_string(),
        endpoint: "/chat/completions".to_string(),
        completion_window: "24h".to_string(),
        metadata: None,
    };
    
    match client.batches().create(request).await {
        Ok(batch) => {
            println!("Batch created: {}", batch.id);
            
            // Check batch status
            match client.batches().retrieve(batch.id.clone()).await {
                Ok(batch_status) => println!("Status: {}", batch_status.status),
                Err(e) => println!("Failed to get batch status: {}", e),
            }
        }
        Err(e) => println!("Failed to create batch: {}", e),
    }
    
    // List batches
    match client.batches().list(None, Some(10)).await {
        Ok(batches) => {
            println!("Found {} batches", batches.data.len());
            for batch in batches.data {
                println!("Batch {}: {}", batch.id, batch.status);
            }
        }
        Err(e) => println!("Failed to list batches: {}", e),
    }
    
    Ok(())
}