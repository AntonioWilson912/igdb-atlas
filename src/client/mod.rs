//! # Client Module
//!
//! The core HTTP client that orchestrates authentication, rate limiting,
//! and endpoint access for the IGDB API.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
//! │   TokenMgr   │<────│  IGDBClient  │────>│  RateLimiter │
//! │    (auth)    │     │  (orchestr.) │     │   (4 req/s)  │
//! └──────────────┘     └──────┬───────┘     └──────────────┘
//!                             │
//!                             │
//!                             v
//!                       ┌───────────┐
//!                       │ Endpoints │
//!                       └───────────┘
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use igdb_atlas::{IGDBClient, ClientConfig};
//! use igdb_atlas::endpoints::traits::{Endpoint, Searchable};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig::new("client_id", "client_secret");
//!     let client = IGDBClient::new(config).await.unwrap();
//!
//!     let games = client.games().search("Zelda").limit(3).execute().await.unwrap();
//!     println!("Found {} games", games.len());
//! }
//! ```

pub mod config;
pub mod rate_limiter;

use std::sync::Arc;

use crate::auth::TokenManager;
use crate::client::config::ClientConfig;
use crate::client::rate_limiter::RateLimiter;
use crate::error::{IGDBError, Result};

/// The base URL for all IGDB API v4 requests.
pub const IGDB_BASE_URL: &str = "https://api.igdb.com/v4";

/// The main client for interacting with the IGDB API.
///
/// Provides access to all endpoint handlers and manages the underlying
/// HTTP client, authentication, and rate limiting.
///
/// ## Endpoint Methods (TODO)
///
/// | Method | Endpoint | Search |
/// |--------|----------|:------:|
///
/// # Cloning
///
/// `IGDBClient` is cheap to clone — all internal state is reference-counted.
/// This makes it easy to share across tasks without `Arc<Mutex<_>>`.
///
/// # Examples
///
/// ```rust,no_run
/// use igdb_atlas::{IGDBClient, ClientConfig};
/// use igdb_atlas::endpoints::traits::Searchable;
///
/// async fn example() {
///     let config = ClientConfig::new("id", "secret");
///     let client = IGDBClient::new(config).await.unwrap();
///
///     // Clone freely for concurrent use
///     let client2 = client.clone();
///     tokio::spawn(async move {
///         let _ = client2.games().search("test").execute().await;
///     });
/// }
/// ```
#[derive(Clone, Debug)]
pub struct IGDBClient {
    base_url: String,
    client_id: String,
    http_client: reqwest::Client,
    rate_limiter: Arc<RateLimiter>,
    token_manager: Arc<TokenManager>,
}

impl IGDBClient {
    // ── Construction ────────────────────────────────────────────────

    /// Creates a new `IGDBClient` from the given configuration.
    ///
    /// This constructor validates the configuration and initializes
    /// all internal components. The OAuth token is fetched lazily
    /// on the first actual API request.
    ///
    /// # Errors
    ///
    /// Returns [`IGDBError::InvalidConfiguration`] if required fields are missing.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::{IGDBClient, ClientConfig};
    ///
    /// async fn example() {
    ///     let config = ClientConfig::new("my_id", "my_secret");
    ///     let client = IGDBClient::new(config).await.unwrap();
    /// }
    /// ```
    pub async fn new(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        let rate_limiter = RateLimiter::new(config.rate_limit_rps, config.max_backoff_attempts);

        let token_manager =
            TokenManager::new(config.client_id.clone(), config.client_secret.clone());

        Ok(Self {
            base_url: config.base_url,
            client_id: config.client_id,
            http_client: reqwest::Client::new(),
            rate_limiter: Arc::new(rate_limiter),
            token_manager: Arc::new(token_manager),
        })
    }

    // ── Raw query ──────────────────────────────────────────────────

    /// Executes a raw Apicalypse query against the specified endpoint path.
    ///
    /// This is the low-level method used by all endpoint handlers.
    /// It handles authentication, rate limiting, and error translation.
    ///
    /// # Generic Parameters
    ///
    /// - `T`: The response model type. Must implement `serde::Deserialize`.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The API endpoint path (e.g., `"games"`, `"platforms"`)
    /// - `query`: The Apicalypse query string (e.g., `"search \"zelda\"; limit 5;"`)
    ///
    /// # Errors
    ///
    /// - [`IGDBError::RateLimited`] if max backoff attempts are exhausted
    /// - [`IGDBError::ApiError`] for non-success HTTP responses
    /// - [`IGDBError::DeserializationError`] if the response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::{IGDBClient, ClientConfig};
    /// use igdb_atlas::models::games::Game;
    ///
    /// async fn example() {
    ///     let config = ClientConfig::new("id", "secret");
    ///     let client = IGDBClient::new(config).await.unwrap();
    ///
    ///     let games: Vec<Game> = client
    ///         .execute_query("games", "search \"zelda\"; limit 5;")
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn execute_query<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &str,
    ) -> Result<T> {
        self.execute_query_internal(endpoint, query).await
    }

    /// Internal method that executes a query and handles the full response lifecycle.
    async fn execute_query_internal<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &str,
    ) -> Result<T> {
        let token = self.token_manager.get_valid_token().await?;
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self
            .rate_limiter
            .execute_with_backoff(|| {
                self.http_client
                    .post(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Client-ID", &self.client_id)
                    .header("Content-Type", "text/plain")
                    .body(query.to_string())
                    .send()
            })
            .await?;

        let status = response.status().as_u16();

        if status == 200 {
            let bytes = response.bytes().await.map_err(|e| IGDBError::ApiError {
                status: 0,
                message: format!("Failed to read response body: {}", e),
            })?;

            serde_json::from_slice(&bytes).map_err(|e| IGDBError::DeserializationError(e))
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(IGDBError::ApiError {
                status,
                message: body,
            })
        }
    }
}
