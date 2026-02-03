//! # External Game Source Model
//!
//! Represents a source or storefront for external game listings from the
//! IGDB v4 `/external_game_sources` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::ExternalGameSource;
//!
//! let json = r#"{"id": 1, "name": "Steam"}"#;
//! let source: ExternalGameSource = serde_json::from_str(json).unwrap();
//! assert_eq!(source.display_name(), "Steam");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A source/storefront record for external game listings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGameSource {
    /// Unique source identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Source/storefront name (e.g. "Steam", "GOG", "Epic Game Store").
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl ExternalGameSource {
    /// Returns the source name or `"Unknown Source"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Source")
    }
}

impl Default for ExternalGameSource {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for ExternalGameSource {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ExternalGameSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExternalGameSource [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", date)?;
            }
        }
        writeln!(f)
    }
}
