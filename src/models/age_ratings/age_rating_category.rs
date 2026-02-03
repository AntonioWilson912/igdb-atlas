//! # Age Rating Category
//!
//! Represents a rating category (e.g. "Mature 17+", "PEGI 18") from the
//! IGDB v4 `/age_rating_categories` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::age_ratings::AgeRatingCategory;
//!
//! let json = r#"{"id": 10, "rating": "Mature 17+", "organization": 1}"#;
//! let cat: AgeRatingCategory = serde_json::from_str(json).unwrap();
//! assert_eq!(cat.rating, Some("Mature 17+".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    age_ratings::AgeRatingOrganization,
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A rating category record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRatingCategory {
    /// Unique rating category identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Reference ID to the `/age_rating_organizations` endpoint.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub organization: Option<AgeRatingOrganization>,

    /// The human-readable rating name (e.g. "Mature 17+").
    #[serde(default)]
    pub rating: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl AgeRatingCategory {
    /// Returns the rating name or `"Unknown Rating"`.
    pub fn display_name(&self) -> &str {
        self.rating.as_deref().unwrap_or("Unknown Rating")
    }
}

impl Default for AgeRatingCategory {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            organization: None,
            rating: None,
            updated_at: None,
        }
    }
}

impl FromId for AgeRatingCategory {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for AgeRatingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgeRatingCategory [{}]", self.id)?;
        if let Some(ref r) = self.rating {
            write!(f, " {}", r)?;
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
