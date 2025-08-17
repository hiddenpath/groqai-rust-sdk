//! Batch processing API implementation
//! 
//! 批处理 API 实现，支持大规模异步任务处理

use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{Batch, BatchList};
use serde::Serialize;

/// Request structure for creating a batch job
/// 
/// This struct contains the parameters needed to create a new batch processing job.
/// Batch jobs allow you to process multiple requests efficiently and cost-effectively.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::api::batches::BatchCreateRequest;
/// 
/// let request = BatchCreateRequest {
///     input_file_id: "file_abc123".to_string(),
///     endpoint: "/chat/completions".to_string(),
///     completion_window: "24h".to_string(),
///     metadata: Some(serde_json::json!({"project": "my_project"})),
/// };
/// ```
#[derive(Serialize, Clone)]
pub struct BatchCreateRequest {
    /// ID of the input file containing the batch requests
    pub input_file_id: String,
    /// The API endpoint to process the batch against
    pub endpoint: String,
    /// Time window for batch completion ("24h" only currently supported)
    pub completion_window: String,
    /// Optional metadata to attach to the batch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Builder for batch processing requests
/// 
/// This builder provides methods for creating, retrieving, listing, and canceling
/// batch processing jobs.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::{GroqClientBuilder, BatchCreateRequest};
/// 
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
/// 
/// // Create a new batch
/// let request = BatchCreateRequest {
///     input_file_id: "file_abc123".to_string(),
///     endpoint: "/chat/completions".to_string(),
///     completion_window: "24h".to_string(),
///     metadata: None,
/// };
/// 
/// let batch = client.batches().create(request).await?;
/// println!("Created batch: {}", batch.id);
/// 
/// // List all batches
/// let batches = client.batches().list(None, None).await?;
/// println!("Found {} batches", batches.data.len());
/// # Ok(())
/// # }
/// ```
pub struct BatchRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> BatchRequestBuilder<'a> {
    /// Creates a new batch request builder
    /// 
    /// # Arguments
    /// 
    /// * `client` - Reference to the GroqClient
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    /// Creates a new batch processing job
    /// 
    /// # Arguments
    /// 
    /// * `req` - The batch creation request parameters
    /// 
    /// # Returns
    /// 
    /// A `Batch` object containing the batch details and status
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the batch creation fails
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use groqai::{GroqClientBuilder, BatchCreateRequest};
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
    /// 
    /// let request = BatchCreateRequest {
    ///     input_file_id: "file_abc123".to_string(),
    ///     endpoint: "/chat/completions".to_string(),
    ///     completion_window: "24h".to_string(),
    ///     metadata: Some(serde_json::json!({"description": "Monthly report generation"})),
    /// };
    /// 
    /// let batch = client.batches().create(request).await?;
    /// println!("Batch {} created with status: {}", batch.id, batch.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(self, req: BatchCreateRequest) -> Result<Batch, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_json("batches", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Retrieves details of a specific batch
    /// 
    /// # Arguments
    /// 
    /// * `batch_id` - The ID of the batch to retrieve
    /// 
    /// # Returns
    /// 
    /// A `Batch` object containing the current batch details and status
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the batch is not found or retrieval fails
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
    /// let batch = client.batches().retrieve("batch_abc123".to_string()).await?;
    /// println!("Batch status: {}", batch.status);
    /// println!("Completed: {}/{}", batch.request_counts.completed, batch.request_counts.total);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(self, batch_id: String) -> Result<Batch, GroqError> {
        let path = format!("batches/{}", batch_id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Lists batch processing jobs with optional pagination
    /// 
    /// # Arguments
    /// 
    /// * `after` - Optional batch ID to start listing after (for pagination)
    /// * `limit` - Optional limit on the number of batches to return (max 100)
    /// 
    /// # Returns
    /// 
    /// A `BatchList` containing the list of batches and pagination information
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
    /// // List first 10 batches
    /// let batches = client.batches().list(None, Some(10)).await?;
    /// for batch in &batches.data {
    ///     println!("Batch {}: {}", batch.id, batch.status);
    /// }
    /// 
    /// // Get next page if available
    /// if batches.has_more {
    ///     let next_batches = client.batches()
    ///         .list(batches.last_id.clone(), Some(10))
    ///         .await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(self, after: Option<String>, limit: Option<u32>) -> Result<BatchList, GroqError> {
        let mut params = Vec::new();
        if let Some(after_id) = after {
            params.push(("after", after_id));
        }
        if let Some(limit_val) = limit {
            params.push(("limit", limit_val.to_string()));
        }
        
        if params.is_empty() {
            let response = self.client.transport.get_json("batches").await?;
            serde_json::from_value(response).map_err(GroqError::from)
        } else {
            let response = self.client.transport.get_with_params("batches", &params).await?;
            serde_json::from_value(response).map_err(GroqError::from)
        }
    }

    /// Cancels a batch processing job
    /// 
    /// # Arguments
    /// 
    /// * `batch_id` - The ID of the batch to cancel
    /// 
    /// # Returns
    /// 
    /// A `Batch` object with the updated status (should be "cancelling" or "cancelled")
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the batch cannot be cancelled or is not found
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
    /// let cancelled_batch = client.batches().cancel("batch_abc123".to_string()).await?;
    /// println!("Batch {} status: {}", cancelled_batch.id, cancelled_batch.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel(self, batch_id: String) -> Result<Batch, GroqError> {
        let path = format!("batches/{}/cancel", batch_id);
        let body = serde_json::Value::Null;
        let response = self.client.transport.post_json(&path, &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}