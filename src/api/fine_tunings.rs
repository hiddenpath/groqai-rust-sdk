//! Fine-tuning API implementation for custom model training
//! 
//! 微调 API 实现，支持自定义模型训练

use crate::client::GroqClient;
use crate::error::GroqError;
use serde::{Deserialize, Serialize};

/// Request structure for creating a fine-tuning job
/// 
/// This struct contains the parameters needed to start a fine-tuning job
/// for creating custom models based on your training data.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::api::fine_tunings::FineTuningCreateRequest;
/// 
/// let request = FineTuningCreateRequest {
///     base_model: "llama-3.1-8b-instant".to_string(),
///     input_file_id: "file_abc123".to_string(),
///     name: "my-custom-model".to_string(),
///     type_: "supervised".to_string(),
/// };
/// ```
#[derive(Serialize, Clone)]
pub struct FineTuningCreateRequest {
    /// Base model to fine-tune from
    pub base_model: String,
    /// ID of the training data file
    pub input_file_id: String,
    /// Name for the fine-tuned model
    pub name: String,
    /// Type of fine-tuning (e.g., "supervised")
    pub type_: String,
}

/// Fine-tuning job details
/// 
/// This struct represents a fine-tuning job and its current status.
#[derive(Deserialize, Debug, Clone)]
pub struct FineTuning {
    /// Unique identifier for the fine-tuning job
    pub id: String,
    /// Name of the fine-tuned model
    pub name: String,
    /// Base model used for fine-tuning
    pub base_model: String,
    /// Type of fine-tuning
    pub type_: String,
    /// ID of the input training file
    pub input_file_id: String,
    /// Timestamp when the job was created
    pub created_at: u64,
    /// Current status of the fine-tuning job
    pub status: String,
    /// ID of the resulting fine-tuned model (if completed)
    pub fine_tuned_model: Option<String>,
    /// Training progress information
    pub training_progress: Option<serde_json::Value>,
    /// Error information if the job failed
    pub error: Option<serde_json::Value>,
}

/// List of fine-tuning jobs
#[derive(Deserialize, Debug, Clone)]
pub struct FineTuningList {
    /// Object type identifier
    pub object: String,
    /// List of fine-tuning jobs
    pub data: Vec<FineTuning>,
    /// Whether there are more results available
    pub has_more: bool,
}

/// Builder for fine-tuning requests
/// 
/// This builder provides methods for creating, retrieving, and listing
/// fine-tuning jobs.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use groqai::{GroqClientBuilder, FineTuningCreateRequest};
/// 
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = GroqClientBuilder::new("gsk_your_api_key".to_string())?.build()?;
/// 
/// let request = FineTuningCreateRequest {
///     base_model: "llama-3.1-8b-instant".to_string(),
///     input_file_id: "file_abc123".to_string(),
///     name: "my-custom-model".to_string(),
///     type_: "supervised".to_string(),
/// };
/// 
/// let fine_tuning = client.fine_tunings().create(request).await?;
/// println!("Started fine-tuning job: {}", fine_tuning.id);
/// # Ok(())
/// # }
/// ```
pub struct FineTuningRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> FineTuningRequestBuilder<'a> {
    /// Creates a new fine-tuning request builder
    /// 
    /// # Arguments
    /// 
    /// * `client` - Reference to the GroqClient
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    /// Creates a new fine-tuning job
    /// 
    /// # Arguments
    /// 
    /// * `req` - The fine-tuning creation request parameters
    /// 
    /// # Returns
    /// 
    /// A `FineTuning` object containing the job details and status
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the fine-tuning job creation fails
    pub async fn create(self, req: FineTuningCreateRequest) -> Result<FineTuning, GroqError> {
        let body = serde_json::to_value(req)?;
        let response = self.client.transport.post_json("fine_tuning/jobs", &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Retrieves details of a specific fine-tuning job
    /// 
    /// # Arguments
    /// 
    /// * `fine_tuning_id` - The ID of the fine-tuning job to retrieve
    /// 
    /// # Returns
    /// 
    /// A `FineTuning` object containing the current job details and status
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the job is not found or retrieval fails
    pub async fn retrieve(self, fine_tuning_id: String) -> Result<FineTuning, GroqError> {
        let path = format!("fine_tuning/jobs/{}", fine_tuning_id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Lists fine-tuning jobs with optional pagination
    /// 
    /// # Arguments
    /// 
    /// * `after` - Optional job ID to start listing after (for pagination)
    /// * `limit` - Optional limit on the number of jobs to return
    /// 
    /// # Returns
    /// 
    /// A `FineTuningList` containing the list of jobs and pagination information
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the listing fails
    pub async fn list(self, after: Option<String>, limit: Option<u32>) -> Result<FineTuningList, GroqError> {
        let mut params = Vec::new();
        if let Some(after_id) = after {
            params.push(("after", after_id));
        }
        if let Some(limit_val) = limit {
            params.push(("limit", limit_val.to_string()));
        }
        
        if params.is_empty() {
            let response = self.client.transport.get_json("fine_tuning/jobs").await?;
            serde_json::from_value(response).map_err(GroqError::from)
        } else {
            let response = self.client.transport.get_with_params("fine_tuning/jobs", &params).await?;
            serde_json::from_value(response).map_err(GroqError::from)
        }
    }

    /// Cancels a fine-tuning job
    /// 
    /// # Arguments
    /// 
    /// * `fine_tuning_id` - The ID of the fine-tuning job to cancel
    /// 
    /// # Returns
    /// 
    /// A `FineTuning` object with the updated status
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the job cannot be cancelled or is not found
    pub async fn cancel(self, fine_tuning_id: String) -> Result<FineTuning, GroqError> {
        let path = format!("fine_tuning/jobs/{}/cancel", fine_tuning_id);
        let body = serde_json::Value::Null;
        let response = self.client.transport.post_json(&path, &body).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}