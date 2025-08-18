// examples/file_management.rs
// File management example

use groqai::{GroqClient, FileCreateRequest};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using environment variables (recommended)
    let client = GroqClient::new()?;
    
    // Upload a file
    let request = FileCreateRequest::new(
        PathBuf::from("training_data.jsonl"),
        "batch".to_string()
    )?;
    
    match client.files().create(request).await {
        Ok(file) => {
            println!("File uploaded: {} (ID: {})", file.filename, file.id);
            
            // Retrieve the file
            match client.files().retrieve(file.id.clone()).await {
                Ok(retrieved_file) => println!("Retrieved file: {}", retrieved_file.filename),
                Err(e) => println!("Failed to retrieve file: {}", e),
            }
            
            // Delete the file
            match client.files().delete(file.id).await {
                Ok(deletion) => println!("File deleted: {}", deletion.deleted),
                Err(e) => println!("Failed to delete file: {}", e),
            }
        }
        Err(e) => println!("Failed to upload file: {}", e),
    }
    
    // List files
    match client.files().list().await {
        Ok(files) => {
            println!("Found {} files", files.data.len());
            for file in files.data {
                println!("File: {} ({})", file.filename, file.purpose);
            }
        }
        Err(e) => println!("Failed to list files: {}", e),
    }
    
    Ok(())
}