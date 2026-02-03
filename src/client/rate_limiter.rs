//! # Rate Limiter
//!
//! Implements proactive rate limiting with exponential backoff for IGDB's
//! 4 requests per second constraint.
//!
//! ## Strategy
//!
//! The rate limiter uses a token bucket approach:
//!
//! 1. **Proactive throttling**: Tracks request timestamps and enforces
//!    minimum intervals between requests based on the configured RPS.
//! 2. **Exponential backoff**: If a 429 response is received, backs off
//!    with exponentially increasing delays plus random jitter.
//! 3. **Maximum attempts**: Gives up after a configurable number of retries.
//!
//! ## Backoff Formula
//!
//! ```text
//! delay = min(base_delay * 2^attempt, max_delay) + random_jitter
//! ```
//!
//! Where:
//! - `base_delay` = 250ms (derived from 4 RPS)
//! - `max_delay` = 32 seconds
//! - `random_jitter` = 0-100ms
//!
//! ## Example
//!
//! ```rust
//! use igdb_atlas::client::rate_limiter::RateLimiter;
//!
//! let limiter = RateLimiter::new(4.0, 10);
//! // Use via execute_with_backoff in client
//! ```

use parking_lot::Mutex;
use rand::Rng;
use std::future::Future;
use std::time::{Duration, Instant};

use crate::error::{IGDBError, Result};

/// Maximum backoff delay cap.
const MAX_BACKOFF_DELAY: Duration = Duration::from_secs(32);

/// Random jitter range in milliseconds.
const JITTER_MAX_MS: u64 = 100;

/// Thread-safe rate limiter with exponential backoff.
///
/// Tracks the last request time and enforces minimum intervals
/// between requests to stay within the API's rate limit.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::client::rate_limiter::RateLimiter;
///
/// let limiter = RateLimiter::new(4.0, 10);
/// assert_eq!(limiter.max_attempts(), 10);
/// ```
#[derive(Debug)]
pub struct RateLimiter {
    /// Minimum time between requests (1/rps seconds).
    min_interval: Duration,
    /// Maximum number of retry attempts on 429.
    max_attempts: u32,
    /// Timestamp of the last request made.
    last_request_time: Mutex<Option<Instant>>,
}

impl RateLimiter {
    /// Creates a new rate limiter.
    ///
    /// # Parameters
    ///
    /// - `rps`: Requests per second limit (e.g., 4.0 for IGDB)
    /// - `max_attempts`: Maximum retry attempts on rate limit errors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::client::rate_limiter::RateLimiter;
    ///
    /// let limiter = RateLimiter::new(4.0, 10);
    /// // min_interval will be 250ms (1/4 seconds)
    /// ```
    pub fn new(rps: f64, max_attempts: u32) -> Self {
        let min_interval = Duration::from_secs_f64(1.0 / rps);
        Self {
            min_interval,
            max_attempts,
            last_request_time: Mutex::new(None),
        }
    }

    /// Returns the maximum number of retry attempts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::client::rate_limiter::RateLimiter;
    ///
    /// let limiter = RateLimiter::new(4.0, 7);
    /// assert_eq!(limiter.max_attempts(), 7);
    /// ```
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Returns the minimum interval between requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::client::rate_limiter::RateLimiter;
    ///
    /// let limiter = RateLimiter::new(4.0, 10);
    /// assert_eq!(limiter.min_interval(), std::time::Duration::from_millis(250));
    /// ```
    pub fn min_interval(&self) -> Duration {
        self.min_interval
    }

    /// Waits if necessary to respect the rate limit, then executes the request.
    ///
    /// If the request returns a 429 status, applies exponential backoff
    /// and retries up to `max_attempts` times.
    ///
    /// The closure must return a `Future` that resolves to a standard
    /// `std::result::Result<reqwest::Response, reqwest::Error>`. The rate
    /// limiter handles the conversion into [`Result`] internally.
    ///
    /// # Generic Parameters
    ///
    /// - `F`: A closure that returns a future performing the HTTP request
    /// - `Fut`: The future type returned by `F`
    ///
    /// # Errors
    ///
    /// - [`IGDBError::RateLimited`] if all retry attempts are exhausted
    /// - [`IGDBError::NetworkError`] if the underlying request fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::client::rate_limiter::RateLimiter;
    ///
    /// async fn example() {
    ///     let limiter = RateLimiter::new(4.0, 10);
    ///     // In practice this is called internally by the client
    /// }
    /// ```
    pub async fn execute_with_backoff<F, Fut>(&self, mut request_fn: F) -> Result<reqwest::Response>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = std::result::Result<reqwest::Response, reqwest::Error>>,
    {
        let mut attempt = 0;

        loop {
            // Proactive throttling: wait if needed
            self.wait_for_rate_limit().await;

            // Execute the request, converting reqwest::Error into IGDBError
            let response = request_fn().await?;

            // Update last request time
            {
                let mut guard = self.last_request_time.lock();
                *guard = Some(Instant::now());
            }

            // If not rate-limited, return the response
            if response.status().as_u16() != 429 {
                return Ok(response);
            }

            // Rate limited - increment attempt
            attempt += 1;
            if attempt >= self.max_attempts {
                let delay = self.compute_backoff_delay(attempt);
                return Err(IGDBError::RateLimited {
                    retry_after_ms: delay.as_millis() as u64,
                    attempts: attempt,
                });
            }

            // Compute and wait for backoff delay
            let delay = self.compute_backoff_delay(attempt);
            log::warn!(
                "Rate limited (attempt {}/{}), backing off for {:?}",
                attempt,
                self.max_attempts,
                delay
            );
            tokio::time::sleep(delay).await;
        }
    }

    /// Waits until enough time has passed since the last request.
    async fn wait_for_rate_limit(&self) {
        let wait_duration = {
            let guard = self.last_request_time.lock();
            match *guard {
                None => None,
                Some(last_time) => {
                    let elapsed = last_time.elapsed();
                    if elapsed < self.min_interval {
                        Some(self.min_interval - elapsed)
                    } else {
                        None
                    }
                }
            }
        };

        if let Some(duration) = wait_duration {
            log::debug!("Rate limit: waiting {:?}", duration);
            tokio::time::sleep(duration).await;
        }
    }

    /// Computes the exponential backoff delay for the given attempt number.
    ///
    /// Formula: `min(base * 2^attempt, max) + jitter`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::client::rate_limiter::RateLimiter;
    ///
    /// let limiter = RateLimiter::new(4.0, 10);
    /// let delay = limiter.compute_backoff_delay(1);
    /// // delay is approximately 250ms * 2^1 = 500ms + jitter
    /// assert!(delay.as_millis() >= 500);
    /// assert!(delay.as_millis() <= 700); // 500 + max jitter 100
    /// ```
    pub fn compute_backoff_delay(&self, attempt: u32) -> Duration {
        let base_ms = self.min_interval.as_millis() as u64;
        let exponential = base_ms.saturating_mul(1u64 << attempt.min(20));
        let capped = exponential.min(MAX_BACKOFF_DELAY.as_millis() as u64);

        let mut rng = rand::thread_rng();
        let jitter: u64 = rng.gen_range(0..=JITTER_MAX_MS);

        Duration::from_millis(capped + jitter)
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            min_interval: self.min_interval,
            max_attempts: self.max_attempts,
            last_request_time: Mutex::new(*self.last_request_time.lock()),
        }
    }
}
