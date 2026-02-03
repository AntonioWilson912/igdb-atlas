//! # Age Rating Organization
//!
//! Represents a rating organization (ESRB, PEGI, CERO, etc.) from the
//! IGDB v4 `/age_rating_organizations` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::age_ratings::AgeRatingOrganization;
//!
//! let json = r#"{"id": 1, "name": "ESRB"}"#;
//! let org: AgeRatingOrganization = serde_json::from_str(json).unwrap();
//! assert_eq!(org.name, Some("ESRB".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A rating organization record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRatingOrganization {
    /// Unique organization identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The organization's name (e.g. "ESRB", "PEGI").
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl AgeRatingOrganization {
    /// Returns the organization name or `"Unknown Organization"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Organization")
    }
}

impl Default for AgeRatingOrganization {
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

impl FromId for AgeRatingOrganization {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for AgeRatingOrganization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgeRatingOrganization [{}]", self.id)?;
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
