//! # Popularity Type Model
//!
//! Represents a popularity type from the IGDB v4
//! `/popularity_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::PopularityType;
//!
//! let json = r#"{
//!     "id": 34,
//!     "popularity_source": 14,
//!     "name": "24hr Hours Watched",
//!     "external_popularity_source": 14
//! }"#;
//! let pp: PopularityType = serde_json::from_str(json).unwrap();
//! assert_eq!(pp.external_popularity_source.as_ref().map(|o| o.id), Some(14));
//! assert_eq!(pp.display_name(), "24hr Hours Watched");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::ExternalGameSource,
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// Available primitives with their source and popularity type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularityType {
    /// Unique primitive type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The external game source.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub external_popularity_source: Option<ExternalGameSource>,

    /// The name of the primitive type.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl PopularityType {
    /// Returns the type name or `"Unknown Type"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Type")
    }
}

impl Default for PopularityType {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            external_popularity_source: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for PopularityType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PopularityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PopularityType [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        if let Some(ts) = self.created_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " created at {}", date)?;
            }
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " updated at {}", date)?;
            }
        }
        writeln!(f)
    }
}
