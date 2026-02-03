//! # Error Handling
//!
//! This module provides custom error types for all failure modes in the IGDB client.
//!
//! ## Error Hierarchy
//!
//! [`IGDBError`] is the top-level error type. It wraps:
//!
//! - Network/transport errors from [`reqwest`]
//! - Authentication failures (token retrieval, expiry)
//! - API errors returned by IGDB (validation, rate limits, server errors)
//! - Serialization/deserialization failures
//! - Query construction errors
//! - Rate limiting with backoff metadata
//!
//! ## Example: Handling specific errors
//!
//! ```rust,no_run
//! use igdb_atlas::{IGDBClient, ClientConfig, IGDBError};
//! use igdb_atlas::endpoints::traits::Searchable;
//!
//! async fn handle_errors() {
//!     let config = ClientConfig::new("id", "secret");
//!     let client = IGDBClient::new(config).await.unwrap();
//!
//!     match client.games().search("test").execute().await {
//!         Ok(games) => println!("Found {} games", games.len()),
//!         Err(IGDBError::RateLimited { retry_after_ms, attempts }) => {
//!             println!("Rate limited! Retry after {}ms (attempt {})", retry_after_ms, attempts);
//!         }
//!         Err(IGDBError::ApiError { status, message }) => {
//!             println!("API error {}: {}", status, message);
//!         }
//!         Err(IGDBError::AuthenticationFailed(msg)) => {
//!             println!("Auth failed: {}", msg);
//!         }
//!         Err(e) => {
//!             println!("Other error: {}", e);
//!         }
//!     }
//! }
//! ```

use std::fmt;

use thiserror::Error;

/// The primary error type for all IGDB client operations.
///
/// Every public method in this crate returns `Result<T, IGDBError>`,
/// giving consumers fine-grained control over error handling.
///
/// # Examples
///
/// Matching on specific error variants:
///
/// ```rust
/// use igdb_atlas::error::IGDBError;
///
/// fn describe_error(err: &IGDBError) -> &'static str {
///     match err {
///         IGDBError::AuthenticationFailed(_) => "auth problem",
///         IGDBError::RateLimited { .. } => "too many requests",
///         IGDBError::ApiError { .. } => "IGDB returned an error",
///         IGDBError::NetworkError(_) => "network issue",
///         IGDBError::DeserializationError(_) => "bad response format",
///         IGDBError::QueryBuildError(_) => "invalid query",
///         IGDBError::TokenExpired => "token needs refresh",
///         IGDBError::InvalidConfiguration(_) => "bad config",
///         IGDBError::Custom(_) => "custom error",
///     }
/// }
/// ```
#[derive(Error, Debug)]
pub enum IGDBError {
    /// Authentication with Twitch OAuth failed.
    ///
    /// This occurs when the client credentials are invalid,
    /// the token endpoint is unreachable, or the response is malformed.
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// The API rate limit (4 req/s) was hit despite backoff attempts.
    ///
    /// Contains the last computed retry delay and how many attempts were made.
    #[error("Rate limited after {attempts} attempts. Retry after {retry_after_ms}ms")]
    RateLimited {
        /// Milliseconds to wait before retrying
        retry_after_ms: u64,
        /// Number of backoff attempts made
        attempts: u32,
    },

    /// IGDB returned an error response (4xx or 5xx).
    #[error("API error (HTTP {status}): {message}")]
    ApiError {
        /// HTTP status code
        status: u16,
        /// Error message from the API
        message: String,
    },

    /// A network/transport-level error from reqwest.
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// Failed to deserialize the API response into a model.
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),

    /// An error constructing or validating an Apicalypse query.
    #[error("Query build error: {0}")]
    QueryBuildError(String),

    /// The cached OAuth token has expired and must be refreshed.
    #[error("Token expired, refresh required")]
    TokenExpired,

    /// Invalid client configuration (missing credentials, bad URLs, etc.).
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// A user-supplied custom error, useful for wrapping external errors.
    #[error("Custom error: {0}")]
    Custom(Box<dyn std::error::Error + Send + Sync>),
}

impl IGDBError {
    /// Creates a custom error wrapping any type that implements `std::error::Error`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::error::IGDBError;
    ///
    /// let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
    /// let igdb_err = IGDBError::from_custom(io_err);
    /// assert!(igdb_err.to_string().contains("disk full"));
    /// ```
    pub fn from_custom<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        IGDBError::Custom(Box::new(err))
    }

    /// Returns `true` if this error represents a retriable condition.
    ///
    /// Currently, rate limit errors and certain network errors are considered retriable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::error::IGDBError;
    ///
    /// let err = IGDBError::RateLimited { retry_after_ms: 1000, attempts: 2 };
    /// assert!(err.is_retriable());
    ///
    /// let err = IGDBError::AuthenticationFailed("bad creds".into());
    /// assert!(!err.is_retriable());
    /// ```
    pub fn is_retriable(&self) -> bool {
        matches!(self, IGDBError::RateLimited { .. })
    }

    /// If this is a rate limit error, returns the recommended retry delay in milliseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::error::IGDBError;
    ///
    /// let err = IGDBError::RateLimited { retry_after_ms: 2500, attempts: 3 };
    /// assert_eq!(err.retry_after_ms(), Some(2500));
    ///
    /// let err = IGDBError::TokenExpired;
    /// assert_eq!(err.retry_after_ms(), None);
    /// ```
    pub fn retry_after_ms(&self) -> Option<u64> {
        match self {
            IGDBError::RateLimited { retry_after_ms, .. } => Some(*retry_after_ms),
            _ => None,
        }
    }
}

/// Display implementation is handled by thiserror derive.
/// This `fmt::Debug` formatting gives a more structured representation
/// useful during development.
impl fmt::LowerHex for IGDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Result type alias for all IGDB operations.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::error::{IGDBError, Result};
///
/// fn parse_id(input: &str) -> Result<u64> {
///     input.parse::<u64>().map_err(|e| IGDBError::QueryBuildError(e.to_string()))
/// }
///
/// assert!(parse_id("42").is_ok());
/// assert!(parse_id("not_a_number").is_err());
/// ```
pub type Result<T> = std::result::Result<T, IGDBError>;
