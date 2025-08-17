//! Rate limiting and retry logic for API requests
//! 
//! 速率限制模块，提供 API 请求的重试和退避机制

use backoff::{backoff::Backoff, ExponentialBackoff};
use std::time::Duration;

/// Rate limiter with exponential backoff for handling API rate limits
/// 
/// This struct provides configuration for retry logic when API requests
/// are rate limited or encounter transient errors.
#[derive(Clone)]
pub struct RateLimiter {
    /// Exponential backoff configuration
    pub backoff: ExponentialBackoff,
}

impl RateLimiter {
    /// Creates a new rate limiter with default settings
    /// 
    /// Default configuration:
    /// - Initial interval: 1 second
    /// - Max interval: 60 seconds
    /// - Multiplier: 2.0
    /// - Max elapsed time: 1 hour
    pub fn new() -> Self {
        Self {
            backoff: ExponentialBackoff {
                initial_interval: Duration::from_secs(1),
                max_interval: Duration::from_secs(60),
                multiplier: 2.0,
                max_elapsed_time: Some(Duration::from_secs(3600)),
                ..Default::default()
            },
        }
    }

    /// Sets the maximum number of retry attempts
    /// 
    /// # Arguments
    /// 
    /// * `attempts` - Maximum number of retry attempts
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.backoff.max_elapsed_time = Some(Duration::from_secs(attempts as u64 * 5));
        self
    }

    /// Resets the backoff state
    pub fn reset(&mut self) {
        self.backoff.reset();
    }

    /// Gets the next backoff duration
    /// 
    /// # Arguments
    /// 
    /// * `retry_after` - Optional retry-after duration from API response
    /// 
    /// # Returns
    /// 
    /// The duration to wait before the next retry attempt
    pub fn next_backoff(&mut self, retry_after: Option<Duration>) -> Option<Duration> {
        retry_after.or_else(|| self.backoff.next_backoff())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}