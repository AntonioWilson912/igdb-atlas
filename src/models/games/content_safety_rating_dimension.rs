//! # Content Safety Rating Dimension Model
//!
//! Represents a dimension for a content safety rating from the
//! IGDB v4 `/content_safety_rating_dimensions` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::ContentSafetyRatingDimension;
//!
//! let json = r#"{"id": 1, "content_safety_rating": 4}"#;
//! let dimension : ContentSafetyRatingDimension = serde_json::from_str(json).unwrap();
//! assert_eq!(dimension.content_safety_rating.as_ref().map(|o| o.id), Some(4));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::ContentSafetyRating,
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A dimension for a content safety rating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSafetyRatingDimension {
    /// Unique source identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The content safety rating.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub content_safety_rating: Option<ContentSafetyRating>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The description of the content safety rating value.
    #[serde(default)]
    pub description: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The content safety rating value
    #[serde(default)]
    pub value: Option<i64>,
}

impl ContentSafetyRatingDimension {
    /// Returns the value or 0.
    pub fn display_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }
}

impl Default for ContentSafetyRatingDimension {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            content_safety_rating: None,
            created_at: None,
            description: None,
            updated_at: None,
            value: None,
        }
    }
}

impl FromId for ContentSafetyRatingDimension {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ContentSafetyRatingDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContentSafetyRatingDimension [{}]", self.id)?;
        if let Some(ref desc) = self.description {
            write!(f, " {}", desc)?;
        }
        if let Some(value) = self.value {
            write!(f, " Value: {}", value)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", date)?;
            }
        }
        writeln!(f)
    }
}
