//! # Collection Relation Type Model
//!
//! Represents a collection relation type from the IGDB v4
//! `/collection_relation_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::collections::CollectionRelationType;
//!
//! let json = r#"{
//!     "id": 2,
//!     "allowed_child_type": 1,
//!     "name": "Spin-off Series"
//! }"#;
//!
//! let collection_relation_type: CollectionRelationType = serde_json::from_str(json).unwrap();
//! assert_eq!(collection_relation_type.name, Some("Spin-off Series".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    collections::CollectionType,
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A collection relation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRelationType {
    /// Unique collection identifier.
    pub id: u64,

    /// The allowed child collection type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub allowed_child_type: Option<CollectionType>,

    /// The allowed parent collection type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub allowed_parent_type: Option<CollectionType>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Description of the relationship type.
    #[serde(default)]
    pub description: Option<String>,

    /// The relationship type name.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl CollectionRelationType {}

impl Default for CollectionRelationType {
    fn default() -> Self {
        Self {
            id: 0,
            allowed_child_type: None,
            allowed_parent_type: None,
            checksum: None,
            created_at: None,
            description: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for CollectionRelationType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CollectionRelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CollectionRelationType [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref description) = self.description {
            writeln!(f, "  Description: {}", description)?;
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
