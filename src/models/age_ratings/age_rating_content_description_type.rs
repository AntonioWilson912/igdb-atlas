//! # Age Rating Content Description Type
//!
//! Represents a content description type (e.g. "Violence", "Language") from
//! the IGDB v4 `/age_rating_content_description_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::age_ratings::AgeRatingContentDescriptionType;
//!
//! let json = r#"{"id": 1, "name": "Violence", "slug": "violence"}"#;
//! let t: AgeRatingContentDescriptionType = serde_json::from_str(json).unwrap();
//! assert_eq!(t.name, Some("Violence".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A content description type record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRatingContentDescriptionType {
    /// Unique content description type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The description type name (e.g. "Violence", "Language").
    #[serde(default)]
    pub name: Option<String>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl AgeRatingContentDescriptionType {
    /// Returns the type name or `"Unknown Type"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Type")
    }
}

impl Default for AgeRatingContentDescriptionType {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            slug: None,
            updated_at: None,
        }
    }
}

impl FromId for AgeRatingContentDescriptionType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for AgeRatingContentDescriptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgeRatingContentDescriptionType [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", d)?;
            }
        }
        writeln!(f)
    }
}
