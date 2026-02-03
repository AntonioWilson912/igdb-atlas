//! # Client Configuration
//!
//! Defines the configuration options for the IGDB client.
//!
//! ## Defaults
//!
//! - Rate limit: 4 requests per second
//! - Max backoff attempts: 10
//! - Base URL: `https://api.igdb.com/v4`
//!
//! ## Example
//!
//! ```rust
//! use igdb_atlas::ClientConfig;
//!
//! let config = ClientConfig::builder()
//!     .client_id("my_id")
//!     .client_secret("my_secret")
//!     .rate_limit_rps(4.0)
//!     .max_backoff_attempts(15)
//!     .build()
//!     .unwrap();
//! ```

use crate::client::IGDB_BASE_URL;
use crate::error::{IGDBError, Result};

/// Configuration for the IGDB client.
///
/// Use [`ClientConfig::new`] for quick setup or [`ClientConfig::builder`]
/// for full control over all options.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::ClientConfig;
///
/// // Quick setup with defaults
/// let config = ClientConfig::new("client_id", "client_secret");
/// assert_eq!(config.client_id, "client_id");
/// assert_eq!(config.rate_limit_rps, 4.0);
/// ```
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Twitch application client ID.
    pub client_id: String,
    /// Twitch application client secret.
    pub client_secret: String,
    /// Maximum requests per second (IGDB limit is 4).
    pub rate_limit_rps: f64,
    /// Maximum number of exponential backoff attempts before giving up.
    pub max_backoff_attempts: u32,
    /// Base URL for the IGDB API.
    pub base_url: String,
}

impl ClientConfig {
    /// Creates a new configuration with sensible defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let config = ClientConfig::new("id123", "secret456");
    /// assert_eq!(config.client_id, "id123");
    /// assert_eq!(config.client_secret, "secret456");
    /// assert_eq!(config.rate_limit_rps, 4.0);
    /// assert_eq!(config.max_backoff_attempts, 10);
    /// ```
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            rate_limit_rps: 4.0,
            max_backoff_attempts: 10,
            base_url: IGDB_BASE_URL.to_string(),
        }
    }

    /// Returns a [`ClientConfigBuilder`] for fluent configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let config = ClientConfig::builder()
    ///     .client_id("my_id")
    ///     .client_secret("my_secret")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }

    /// Validates the configuration, returning an error if invalid.
    ///
    /// # Errors
    ///
    /// - [`IGDBError::InvalidConfiguration`] if client_id or client_secret is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let config = ClientConfig::new("id", "secret");
    /// assert!(config.validate().is_ok());
    ///
    /// let bad = ClientConfig::new("", "secret");
    /// assert!(bad.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.client_id.is_empty() {
            return Err(IGDBError::InvalidConfiguration(
                "client_id cannot be empty".to_string(),
            ));
        }
        if self.client_secret.is_empty() {
            return Err(IGDBError::InvalidConfiguration(
                "client_secret cannot be empty".to_string(),
            ));
        }
        if self.rate_limit_rps <= 0.0 {
            return Err(IGDBError::InvalidConfiguration(
                "rate_limit_rps must be positive".to_string(),
            ));
        }
        if self.max_backoff_attempts == 0 {
            return Err(IGDBError::InvalidConfiguration(
                "max_backoff_attempts must be at least 1".to_string(),
            ));
        }
        Ok(())
    }
}

/// Builder for [`ClientConfig`] with fluent API.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::ClientConfig;
///
/// let config = ClientConfig::builder()
///     .client_id("id")
///     .client_secret("secret")
///     .rate_limit_rps(2.0)
///     .max_backoff_attempts(5)
///     .build()
///     .unwrap();
///
/// assert_eq!(config.rate_limit_rps, 2.0);
/// ```
#[derive(Debug, Default)]
pub struct ClientConfigBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    rate_limit_rps: Option<f64>,
    max_backoff_attempts: Option<u32>,
    base_url: Option<String>,
}

impl ClientConfigBuilder {
    /// Sets the Twitch client ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let builder = ClientConfig::builder().client_id("my_app_id");
    /// ```
    pub fn client_id(mut self, id: &str) -> Self {
        self.client_id = Some(id.to_string());
        self
    }

    /// Sets the Twitch client secret.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let builder = ClientConfig::builder().client_secret("my_secret");
    /// ```
    pub fn client_secret(mut self, secret: &str) -> Self {
        self.client_secret = Some(secret.to_string());
        self
    }

    /// Sets the rate limit in requests per second.
    ///
    /// Default is 4.0 (IGDB's documented limit).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let builder = ClientConfig::builder().rate_limit_rps(3.0);
    /// ```
    pub fn rate_limit_rps(mut self, rps: f64) -> Self {
        self.rate_limit_rps = Some(rps);
        self
    }

    /// Sets the maximum number of exponential backoff attempts.
    ///
    /// Default is 10.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let builder = ClientConfig::builder().max_backoff_attempts(20);
    /// ```
    pub fn max_backoff_attempts(mut self, attempts: u32) -> Self {
        self.max_backoff_attempts = Some(attempts);
        self
    }

    /// Sets a custom base URL (useful for testing or proxies).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let builder = ClientConfig::builder().base_url("http://localhost:8080");
    /// ```
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    /// Builds the [`ClientConfig`], returning an error if required fields are missing.
    ///
    /// # Errors
    ///
    /// Returns [`IGDBError::InvalidConfiguration`] if client_id or client_secret
    /// are not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::ClientConfig;
    ///
    /// let result = ClientConfig::builder().build();
    /// assert!(result.is_err()); // Missing required fields
    ///
    /// let config = ClientConfig::builder()
    ///     .client_id("id")
    ///     .client_secret("secret")
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(config.client_id, "id");
    /// ```
    pub fn build(self) -> Result<ClientConfig> {
        let client_id = self
            .client_id
            .ok_or_else(|| IGDBError::InvalidConfiguration("client_id is required".to_string()))?;

        let client_secret = self.client_secret.ok_or_else(|| {
            IGDBError::InvalidConfiguration("client_secret is required".to_string())
        })?;

        Ok(ClientConfig {
            client_id,
            client_secret,
            rate_limit_rps: self.rate_limit_rps.unwrap_or(4.0),
            max_backoff_attempts: self.max_backoff_attempts.unwrap_or(10),
            base_url: self.base_url.unwrap_or_else(|| IGDB_BASE_URL.to_string()),
        })
    }
}
