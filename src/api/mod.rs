//! API endpoint implementations for Groq services
//! 
//! API 端点实现模块，包含所有 Groq 服务的接口

/// Chat completion API endpoints and request builders
pub mod chat;

/// Audio transcription and translation API endpoints
pub mod audio;

/// Batch processing API endpoints for efficient bulk operations
pub mod batches;

/// File management API endpoints for upload, list, retrieve, and delete operations
pub mod files;

/// Model information API endpoints for listing and retrieving model details
pub mod models;

/// Fine-tuning API endpoints for custom model training
pub mod fine_tunings;