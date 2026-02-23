//! # Content Safety Rating Model
//!
//! Represents a content safety rating from the
//! IGDB v4 `/content_safety_ratings` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::ContentSafetyRating;
//!
//! let json = r#"{"id": 1, "name": "E"}"#;
//! let rating : ContentSafetyRating = serde_json::from_str(json).unwrap();
//! assert_eq!(rating.display_name(), "E");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A content safety rating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSafetyRating {
    /// Unique source identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The name of the content safety rating.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl ContentSafetyRating {
    /// Returns the rating name or `"Unknown Rating"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Rating")
    }
}

impl Default for ContentSafetyRating {
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

impl FromId for ContentSafetyRating {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ContentSafetyRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContentSafetyRating [{}]", self.id)?;
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
