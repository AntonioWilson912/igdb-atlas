//! # Region Model
//!
//! Represents a game localization region from the IGDB v4 `/regions` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::Region;
//!
//! let json = r#"{
//!     "id": 48,
//!     "name": "Europe",
//!     "category": "continent",
//!     "identifier": "EU"
//! }"#;
//!
//! let region: Region = serde_json::from_str(json).unwrap();
//! assert_eq!(region.name, Some("Europe".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A region for game localization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    /// Unique region identifier.
    pub id: u64,

    /// This can be either 'locale' or 'continent'.
    #[serde(default)]
    pub category: Option<String>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The identifier of each region.
    #[serde(default)]
    pub identifier: Option<String>,

    /// The region's display name.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Region {
    /// Returns the region name or `"Unknown Region"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Region")
    }
}

impl Default for Region {
    fn default() -> Self {
        Self {
            id: 0,
            category: None,
            checksum: None,
            created_at: None,
            identifier: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for Region {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Region [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref category) = self.category {
            writeln!(f, "  Category: {}", category)?;
        }
        if let Some(ref identifier) = self.identifier {
            writeln!(f, "  Identifier: {}", identifier)?;
        }
        if let Some(ts) = self.created_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Added: {}", date)?;
            }
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Updated: {}", date)?;
            }
        }
        Ok(())
    }
}
