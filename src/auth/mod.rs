//! # Authentication Module
//!
//! Handles Twitch OAuth 2.0 client credentials flow for IGDB API access.
//!
//! ## Overview
//!
//! The IGDB API requires a valid Twitch OAuth token. This module implements:
//!
//! - **Token retrieval** via the Twitch token endpoint
//! - **Token caching** to avoid redundant requests
//! - **Transparent refresh** when tokens approach expiry
//! - **Thread-safe access** using `parking_lot` mutexes
//!
//! ## Token Lifecycle
//!
//! ```text
//! [No Token] ──> [Fetch Token] ──> [Cache Token]
//!                                      │
//!                              ┌───────┴────────┐
//!                              v                v
//!                        [Token Valid]   [Token Expiring]
//!                              │                │
//!                              v                v
//!                         [Use Token]     [Refresh Token]
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use igdb_atlas::auth::TokenManager;
//!
//! async fn auth_example() {
//!     let manager = TokenManager::new(
//!         "your_client_id".to_string(),
//!         "your_client_secret".to_string(),
//!     );
//!
//!     // First call fetches and caches
//!     let token = manager.get_valid_token().await.unwrap();
//!     println!("Token: {}", token);
//!
//!     // Subsequent calls return cached token (or refresh if needed)
//!     let token = manager.get_valid_token().await.unwrap();
//!     println!("Cached token: {}", token);
//! }
//! ```

pub mod token_manager;

pub use token_manager::{OAuthToken, TokenManager};
