//! # Collection Relation Model
//!
//! Represents a relationship between collections from the IGDB v4
//! `/collection_relations` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::collections::CollectionRelation;
//!
//! let json = r#"{
//!     "id": 2,
//!     "child_collection": 2,
//!     "parent_collection": 5
//! }"#;
//!
//! let collection_relation: CollectionRelation = serde_json::from_str(json).unwrap();
//! assert_eq!(collection_relation.child_collection.as_ref().map(|o| o.id), Some(2));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    collections::{Collection, CollectionRelationType},
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A collection relation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRelation {
    /// Unique collection identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The child collection of the collection.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub child_collection: Option<Collection>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The parent collection of this collection.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub parent_collection: Option<Collection>,

    /// The collection relationship type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub r#type: Option<CollectionRelationType>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl CollectionRelation {}

impl Default for CollectionRelation {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            child_collection: None,
            created_at: None,
            parent_collection: None,
            r#type: None,
            updated_at: None,
        }
    }
}

impl FromId for CollectionRelation {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CollectionRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CollectionRelation [{}]", self.id)?;
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
