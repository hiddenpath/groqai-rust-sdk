use backoff::{backoff::Backoff, ExponentialBackoff};
use std::time::Duration;

#[derive(Clone)]
pub struct RateLimiter {
    pub backoff: ExponentialBackoff,
}

impl RateLimiter {
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

    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.backoff.max_elapsed_time = Some(Duration::from_secs(attempts as u64 * 5));
        self
    }

    pub fn reset(&mut self) {
        self.backoff.reset();
    }

    pub fn next_backoff(&mut self, retry_after: Option<Duration>) -> Option<Duration> {
        retry_after.or_else(|| self.backoff.next_backoff())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
