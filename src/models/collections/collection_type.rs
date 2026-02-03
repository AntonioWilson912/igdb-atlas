//! # Collection Type Model
//!
//! Represents a collection type from the IGDB v4
//! `/collection_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::collections::CollectionType;
//!
//! let json = r#"{
//!     "id": 1,
//!     "name": "Series"
//! }"#;
//!
//! let collection_type: CollectionType = serde_json::from_str(json).unwrap();
//! assert_eq!(collection_type.name, Some("Series".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A collection type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionType {
    /// Unique collection identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Description of the collection type.
    #[serde(default)]
    pub description: Option<String>,

    /// The name of the collection type.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl CollectionType {}

impl Default for CollectionType {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            description: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for CollectionType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CollectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CollectionType [{}]", self.id)?;
        if let Some(ts) = self.created_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Added: {}", date)?;
            }
        }
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref description) = self.description {
            writeln!(f, "  Description: {}", description)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Updated: {}", date)?;
            }
        }
        Ok(())
    }
}
