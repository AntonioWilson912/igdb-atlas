//! # Release Date Region Model
//!
//! Represents a release date region from the IGDB v4
//! `/release_date_regions` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::ReleaseDateRegion;
//!
//! let json = r#"{"id": 2, "region": "North America"}"#;
//! let region: ReleaseDateRegion = serde_json::from_str(json).unwrap();
//! assert_eq!(region.display_name(), "North America");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A release date region record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDateRegion {
    /// Unique region identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Region name (e.g. "North America", "Europe", "Worldwide").
    #[serde(default)]
    pub region: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl ReleaseDateRegion {
    /// Returns the region name or `"Unknown Region"`.
    pub fn display_name(&self) -> &str {
        self.region.as_deref().unwrap_or("Unknown Region")
    }
}

impl Default for ReleaseDateRegion {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            region: None,
            updated_at: None,
        }
    }
}

impl FromId for ReleaseDateRegion {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ReleaseDateRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReleaseDateRegion [{}]", self.id)?;
        if let Some(ref r) = self.region {
            write!(f, " {}", r)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", d)?;
            }
        }
        writeln!(f)
    }
}
