//! File management API implementation
//! 
//! 文件管理 API 实现，支持文件上传、列表、检索和删除操作

use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{WorkFile, WorkFileList, WorkFileDeletion};
use serde::Serialize;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;

/// Request structure for creating/uploading a file
/// 
/// This struct contains the parameters needed to upload a file to Groq.
/// Files must be in JSONL format and are typically used for batch processing
/// or fine-tuning operations.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::api::files::FileCreateRequest;
/// use std::path::PathBuf;
/// 
/// let request = FileCreateRequest::new(
///     PathBuf::from("training_data.jsonl"),
///     "batch".to_string()
/// )?;
/// # Ok::<(), groqai::GroqError>(())
/// ```
#[derive(Serialize, Clone)]
pub struct FileCreateRequest {
    /// Path to the file to upload
    pub file: PathBuf,
    /// Purpose of the file (e.g., "batch", "fine-tune")
    pub purpose: String,
}

impl FileCreateRequest {
    /// Creates a new file upload request with validation
    /// 
    /// This method validates that the file exists, has the correct extension (.jsonl),
    /// and contains valid JSON lines.
    /// 
    /// # Arguments
    /// 
    /// * `file` - Path to the JSONL file to upload
    /// * `purpose` - Purpose of the file (e.g., "batch", "fine-tune")
    /// 
    /// # Returns
    /// 
    /// A validated `FileCreateRequest` instance
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError::InvalidMessage` if:
    /// - File doesn't have .jsonl extension
    /// - File cannot be opened or read
    /// - File contains invalid JSON lines
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::api::files::FileCreateRequest;
    /// use std::path::PathBuf;
    /// 
    /// // Valid JSONL file for batch processing
    /// let request = FileCreateRequest::new(
    ///     PathBuf::from("batch_requests.jsonl"),
    ///     "batch".to_string()
    /// )?;
    /// 
    /// // This would fail - wrong extension
    /// let invalid_request = FileCreateRequest::new(
    ///     PathBuf::from("data.txt"),
    ///     "batch".to_string()
    /// );
    /// assert!(invalid_request.is_err());
    /// # Ok::<(), groqai::GroqError>(())
    /// ```
    pub fn new(file: PathBuf, purpose: String) -> Result<Self, GroqError> {
        // Validate file extension
        if file.extension().and_then(|ext| ext.to_str()) != Some("jsonl") {
            return Err(GroqError::InvalidMessage(
                "File must have .jsonl extension".to_string(),
            ));
        }

        // Validate file content (each line must be valid JSON)
        let file_reader = File::open(&file)
            .map_err(|e| GroqError::InvalidMessage(format!("Failed to open file: {}", e)))?;
        let reader = BufReader::new(file_reader);
        for (index, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| {
                GroqError::InvalidMessage(format!("Failed to read line {}: {}", index + 1, e))
            })?;
            if !line.trim().is_empty() {
                serde_json::from_str::<serde_json::Value>(&line).map_err(|e| {
                    GroqError::InvalidMessage(format!("Invalid JSONL at line {}: {}", index + 1, e))
                })?;
            }
        }

        Ok(Self { file, purpose })
    }
}

/// Builder for file management requests
/// 
/// This builder provides methods for uploading, listing, retrieving, and deleting
/// files in your Groq account.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::{GroqClientBuilder, FileCreateRequest};
/// use std::path::PathBuf;
/// 
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
/// 
/// // Upload a file
/// let request = FileCreateRequest::new(
///     PathBuf::from("data.jsonl"),
///     "batch".to_string()
/// )?;
/// let file = client.files().create(request).await?;
/// println!("Uploaded file: {}", file.id);
/// 
/// // List all files
/// let files = client.files().list().await?;
/// println!("Found {} files", files.data.len());
/// # Ok(())
/// # }
/// ```
pub struct FileRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> FileRequestBuilder<'a> {
    /// Creates a new file request builder
    /// 
    /// # Arguments
    /// 
    /// * `client` - Reference to the GroqClient
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    /// Uploads a file to Groq
    /// 
    /// # Arguments
    /// 
    /// * `req` - The file upload request containing file path and purpose
    /// 
    /// # Returns
    /// 
    /// A `WorkFile` object containing the uploaded file's details
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the upload fails or file validation fails
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{GroqClientBuilder, FileCreateRequest};
    /// use std::path::PathBuf;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let request = FileCreateRequest::new(
    ///     PathBuf::from("training_data.jsonl"),
    ///     "fine-tune".to_string()
    /// )?;
    /// 
    /// let file = client.files().create(request).await?;
    /// println!("File uploaded: {} ({} bytes)", file.filename, file.bytes);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(self, req: FileCreateRequest) -> Result<WorkFile, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_multipart("files", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Lists all files in your account
    /// 
    /// # Returns
    /// 
    /// A `WorkFileList` containing all files and their metadata
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the listing fails
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let files = client.files().list().await?;
    /// for file in files.data {
    ///     println!("File: {} ({})", file.filename, file.purpose);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(self) -> Result<WorkFileList, GroqError> {
        let response = self.client.transport.get_json("files").await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Retrieves details of a specific file
    /// 
    /// # Arguments
    /// 
    /// * `file_id` - The ID of the file to retrieve
    /// 
    /// # Returns
    /// 
    /// A `WorkFile` object containing the file's details
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the file is not found or retrieval fails
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let file = client.files().retrieve("file_abc123".to_string()).await?;
    /// println!("File: {} ({} bytes)", file.filename, file.bytes);
    /// println!("Created: {}", file.created_at);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(self, file_id: String) -> Result<WorkFile, GroqError> {
        let path = format!("files/{}", file_id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Deletes a file from your account
    /// 
    /// # Arguments
    /// 
    /// * `file_id` - The ID of the file to delete
    /// 
    /// # Returns
    /// 
    /// A `WorkFileDeletion` object confirming the deletion
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the file cannot be deleted or is not found
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::GroqClientBuilder;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let deletion = client.files().delete("file_abc123".to_string()).await?;
    /// if deletion.deleted {
    ///     println!("File {} successfully deleted", deletion.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(self, file_id: String) -> Result<WorkFileDeletion, GroqError> {
        let path = format!("files/{}", file_id);
        let response = self.client.transport.delete_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}