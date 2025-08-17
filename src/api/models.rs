//! Models API implementation for retrieving model information
//! 
//! 模型 API 实现，用于获取可用模型信息

use crate::client::GroqClient;
use crate::error::GroqError;
use crate::types::{Model, ModelList};

/// Builder for model information requests
/// 
/// This builder provides methods for listing available models and retrieving
/// detailed information about specific models.
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
/// // List all available models
/// let models = client.models().list().await?;
/// for model in models.data {
///     println!("Model: {} ({})", model.id, model.owned_by);
/// }
/// 
/// // Get details of a specific model
/// let model = client.models().retrieve("llama-3.1-70b-versatile".to_string()).await?;
/// println!("Context window: {} tokens", model.context_window);
/// # Ok(())
/// # }
/// ```
pub struct ModelsRequestBuilder<'a> {
    client: &'a GroqClient,
}

impl<'a> ModelsRequestBuilder<'a> {
    /// Creates a new models request builder
    /// 
    /// # Arguments
    /// 
    /// * `client` - Reference to the GroqClient
    pub fn new(client: &'a GroqClient) -> Self {
        Self { client }
    }

    /// Lists all available models
    /// 
    /// # Returns
    /// 
    /// A `ModelList` containing all available models and their basic information
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the request fails
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
    /// let models = client.models().list().await?;
    /// println!("Available models:");
    /// for model in models.data {
    ///     println!("  {} - {} ({})", 
    ///         model.id, 
    ///         if model.active { "Active" } else { "Inactive" },
    ///         model.owned_by
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(self) -> Result<ModelList, GroqError> {
        let response = self.client.transport.get_json("models").await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }

    /// Retrieves detailed information about a specific model
    /// 
    /// # Arguments
    /// 
    /// * `model_id` - The ID of the model to retrieve (e.g., "llama-3.1-70b-versatile")
    /// 
    /// # Returns
    /// 
    /// A `Model` object containing detailed information about the model
    /// 
    /// # Errors
    /// 
    /// Returns `GroqError` if the model is not found or the request fails
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
    /// let model = client.models().retrieve("llama-3.1-70b-versatile".to_string()).await?;
    /// println!("Model: {}", model.id);
    /// println!("Owner: {}", model.owned_by);
    /// println!("Context Window: {} tokens", model.context_window);
    /// println!("Active: {}", model.active);
    /// println!("Created: {}", model.created);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(self, model_id: String) -> Result<Model, GroqError> {
        let path = format!("models/{}", model_id);
        let response = self.client.transport.get_json(&path).await?;
        serde_json::from_value(response).map_err(GroqError::from)
    }
}