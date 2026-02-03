//! # igdb-atlas
//!
//! An asynchronous Rust wrapper for the [IGDB v4 API](https://api-docs.igdb.com),
//! the video game database by Twitch.
//!
//! ## Features
//!
//! - Full async/await support via [`tokio`]
//! - Twitch OAuth 2.0 client credentials authentication with automatic token refresh
//! - Proactive rate limiting with exponential backoff (4 req/s limit)
//! - Type-safe query builder supporting IGDB's Apicalypse syntax
//! - Custom error types with detailed context
//!
//! ## Module Structure
//!
//! - [`auth`] — Twitch OAuth 2.0 authentication and token management
//! - [`client`] — Core HTTP client, configuration, and rate limiting
//! - [`error`] — Custom error types
//! - [`query`] — Apicalypse query builder

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod query;

pub use client::IGDBClient;
pub use client::config::ClientConfig;
pub use error::IGDBError;
pub use query::QueryBuilder;
