# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- TBD

## [0.1.10] - 2024-12-19

### Added
- **Convenience Methods for Client Creation**:
  - `GroqClient::new()` - Create client from environment variables (recommended)
  - `GroqClient::from_env()` - Explicit environment variable client creation
  - `GroqClient::with_api_key()` - Direct API key client creation
- **Environment Variable Support**:
  - `GROQ_API_KEY` (required) - Your Groq API key
  - `GROQ_PROXY_URL` / `HTTPS_PROXY` / `HTTP_PROXY` (optional) - Proxy configuration
  - `GROQ_TIMEOUT_SECS` (optional, default: 30) - Request timeout in seconds
- **Enhanced Import System**:
  - `groqai::prelude::*` - Convenient import for common types (recommended for learning)
  - Organized public API exports with clear categorization
  - Three-tier import strategy for different use cases
- **New Examples**:
  - `client_convenience.rs` - Basic convenience methods demonstration
  - `client_creation_methods.rs` - Comprehensive client creation guide
  - `import_patterns.rs` - Different import patterns and best practices
- **Enhanced Testing**: Complete unit test coverage for all convenience methods

### Changed
- **Documentation Overhaul**:
  - Updated `lib.rs` Quick Start to showcase new convenience methods and import patterns
  - Enhanced README.md with environment variable configuration and import patterns
  - Complete Chinese translation (README_CN.md) with all new features
  - All examples updated to use recommended `GroqClient::new()` method
- **Improved Developer Experience**: 
  - Reduced client creation from 5 steps to 1 step for common use cases
  - Three-tier import strategy: prelude for learning, specific for applications, granular for libraries
  - Clear usage recommendations for different scenarios

### Deprecated
- None (all changes are backward compatible)

### Security
- Environment variable-based configuration improves security by avoiding hardcoded API keys

## [0.1.9] - 2024-01-XX

### Features
- **Core API Support**:
  - Chat Completions (streaming & non-streaming)
  - Audio Transcription & Translation with Whisper models
  - File Management (upload, list, retrieve, delete)
  - Batch Processing for efficient bulk operations
  - Model Information retrieval
  - Fine-tuning Support
- **Enterprise Features**:
  - Smart rate limiting with exponential backoff
  - Comprehensive error handling with detailed error types
  - HTTP/HTTPS proxy support
  - Configurable timeouts and retry mechanisms
- **Developer Experience**:
  - Type-safe API with compile-time guarantees
  - Async/await support built on Tokio
  - Rich documentation and examples