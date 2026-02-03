//! # Token Manager
//!
//! Manages the full lifecycle of Twitch OAuth tokens including
//! fetching, caching, and proactive refresh.
//!
//! The token manager uses a configurable refresh threshold (default 60 seconds)
//! to proactively refresh tokens before they actually expire, preventing
//! authentication errors during active use.

use parking_lot::Mutex;
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::error::{IGDBError, Result};

/// The Twitch token endpoint URL.
const TWITCH_TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";

/// How many seconds before expiry to trigger a refresh.
const REFRESH_THRESHOLD_SECS: u64 = 60;

/// Response structure from the Twitch OAuth token endpoint.
///
/// # Examples
///
/// ```rust
/// use serde_json;
/// use igdb_atlas::auth::OAuthToken;
///
/// let json = r#"{
///     "access_token": "abc123",
///     "expires_in": 3600,
///     "token_type": "bearer"
/// }"#;
///
/// let token: OAuthToken = serde_json::from_str(json).unwrap();
/// assert_eq!(token.access_token, "abc123");
/// assert_eq!(token.expires_in, 3600);
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct OAuthToken {
    /// The bearer token string to use in Authorization headers.
    pub access_token: String,
    /// Lifetime of the token in seconds.
    pub expires_in: u64,
    /// Token type, should always be "bearer".
    pub token_type: String,
}

/// Internal cached token with expiry tracking.
#[derive(Debug, Clone)]
struct CachedToken {
    token: OAuthToken,
    fetched_at: Instant,
}

impl CachedToken {
    /// Returns `true` if the token will expire within the refresh threshold.
    fn needs_refresh(&self) -> bool {
        let elapsed = self.fetched_at.elapsed();
        let lifetime = Duration::from_secs(self.token.expires_in);
        let threshold = Duration::from_secs(REFRESH_THRESHOLD_SECS);

        elapsed + threshold >= lifetime
    }

    /// Returns `true` if the token is completely expired.
    fn is_expired(&self) -> bool {
        let elapsed = self.fetched_at.elapsed();
        let lifetime = Duration::from_secs(self.token.expires_in);
        elapsed >= lifetime
    }
}

/// Thread-safe manager for Twitch OAuth tokens.
///
/// Handles fetching, caching, and refreshing tokens transparently.
/// All methods are async and safe to call from multiple tasks concurrently.
///
/// # Examples
///
/// ```rust,no_run
/// use igdb_atlas::auth::TokenManager;
///
/// async fn example() {
///     let manager = TokenManager::new(
///         "client_id".to_string(),
///         "client_secret".to_string(),
///     );
///
///     // This fetches on first call, then returns cached
///     let token = manager.get_valid_token().await.unwrap();
///     println!("Got token: {}", token);
/// }
/// ```
#[derive(Debug)]
pub struct TokenManager {
    client_id: String,
    client_secret: String,
    cached_token: Arc<Mutex<Option<CachedToken>>>,
    http_client: reqwest::Client,
}

impl TokenManager {
    /// Creates a new `TokenManager` with the given Twitch client credentials.
    ///
    /// The token is NOT fetched at construction time; it will be fetched
    /// lazily on the first call to [`get_valid_token`](Self::get_valid_token).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::auth::TokenManager;
    ///
    /// let manager = TokenManager::new(
    ///     "my_client_id".to_string(),
    ///     "my_client_secret".to_string(),
    /// );
    /// ```
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            cached_token: Arc::new(Mutex::new(None)),
            http_client: reqwest::Client::new(),
        }
    }

    /// Returns a valid access token, fetching or refreshing as needed.
    ///
    /// This method is the primary interface for consumers. It handles
    /// all caching logic internally:
    ///
    /// 1. If no token is cached -> fetch a new one
    /// 2. If cached token needs refresh -> fetch a new one
    /// 3. Otherwise -> return the cached token
    ///
    /// # Errors
    ///
    /// Returns [`IGDBError::AuthenticationFailed`] if the token request fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::auth::TokenManager;
    ///
    /// async fn example() {
    ///     let manager = TokenManager::new("id".into(), "secret".into());
    ///     let token = manager.get_valid_token().await.unwrap();
    ///     // token is now a valid bearer token string
    /// }
    /// ```
    pub async fn get_valid_token(&self) -> Result<String> {
        let needs_fetch = {
            let guard = self.cached_token.lock();
            match guard.as_ref() {
                None => true,
                Some(cached) => cached.needs_refresh(),
            }
        };

        if needs_fetch {
            let new_token = self.fetch_token().await?;
            let mut guard = self.cached_token.lock();
            *guard = Some(CachedToken {
                token: new_token.clone(),
                fetched_at: Instant::now(),
            });
            Ok(new_token.access_token)
        } else {
            let guard = self.cached_token.lock();
            Ok(guard.as_ref().unwrap().token.access_token.clone())
        }
    }

    /// Returns `true` if a cached token exists and is not expired.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::auth::TokenManager;
    ///
    /// let manager = TokenManager::new("id".into(), "secret".into());
    /// // No token fetched yet
    /// assert!(!manager.has_valid_token());
    /// ```
    pub fn has_valid_token(&self) -> bool {
        let guard = self.cached_token.lock();
        match guard.as_ref() {
            None => false,
            Some(cached) => !cached.is_expired(),
        }
    }

    /// Invalidates the cached token, forcing a fresh fetch on next use.
    ///
    /// Useful if you suspect the token has been revoked externally.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::auth::TokenManager;
    ///
    /// let manager = TokenManager::new("id".into(), "secret".into());
    /// manager.invalidate_token();
    /// assert!(!manager.has_valid_token());
    /// ```
    pub fn invalidate_token(&self) {
        let mut guard = self.cached_token.lock();
        *guard = None;
    }

    /// Fetches a new OAuth token from Twitch.
    ///
    /// Uses the client credentials grant type, which requires only
    /// a client_id and client_secret (no user interaction).
    async fn fetch_token(&self) -> Result<OAuthToken> {
        let response = self
            .http_client
            .post(TWITCH_TOKEN_URL)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "client_credentials"),
            ])
            .send()
            .await
            .map_err(|e| {
                IGDBError::AuthenticationFailed(format!("Failed to reach token endpoint: {}", e))
            })?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(IGDBError::AuthenticationFailed(format!(
                "Token request failed with status {}: {}",
                status, body
            )));
        }

        response.json::<OAuthToken>().await.map_err(|e| {
            IGDBError::AuthenticationFailed(format!("Failed to parse token response: {}", e))
        })
    }
}

impl Clone for TokenManager {
    fn clone(&self) -> Self {
        Self {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            cached_token: self.cached_token.clone(),
            http_client: self.http_client.clone(),
        }
    }
}
