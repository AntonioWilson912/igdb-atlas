//! # Age Rating Content Description V2
//!
//! Represents a per-rating content flag from the IGDB v4
//! `/age_rating_content_descriptions_v2` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::age_ratings::AgeRatingContentDescriptionV2;
//!
//! let json = r#"{"id": 1, "description": "Blood and Gore", "organization": 1, "description_type": 3}"#;
//! let desc: AgeRatingContentDescriptionV2 = serde_json::from_str(json).unwrap();
//! assert_eq!(desc.description, Some("Blood and Gore".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    age_ratings::{AgeRatingContentDescriptionType, AgeRatingOrganization},
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A content description record linking a textual flag to an
/// organization and description type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRatingContentDescriptionV2 {
    /// Unique content description identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Textual description of the content flag.
    #[serde(default)]
    pub description: Option<String>,

    /// The age rating content description type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub description_type: Option<AgeRatingContentDescriptionType>,

    /// The rating organization.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub organization: Option<AgeRatingOrganization>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl AgeRatingContentDescriptionV2 {
    /// Returns the description text or `"Unknown Description"`.
    pub fn display_name(&self) -> &str {
        self.description.as_deref().unwrap_or("Unknown Description")
    }
}

impl Default for AgeRatingContentDescriptionV2 {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            description: None,
            description_type: None,
            organization: None,
            updated_at: None,
        }
    }
}

impl FromId for AgeRatingContentDescriptionV2 {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for AgeRatingContentDescriptionV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgeRatingContentDescriptionV2 [{}]", self.id)?;
        if let Some(ref d) = self.description {
            write!(f, " {}", d)?;
        }
        if let Some(ref org) = self.organization {
            write!(f, " (org={})", org)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " updated {}", d)?;
            }
        }
        writeln!(f)
    }
}
