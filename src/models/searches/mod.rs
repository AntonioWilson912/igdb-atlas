//! # Search Models
//!
//! Models for searching characters, collections, companies, games, platforms, and themes
//! from the IGDB `/search` endpoint.
//!
//! > **Note**: The `company` field is listed in the IGDB spec but appears
//! > unpopulated in practice.

pub mod search;

pub use search::Search;
