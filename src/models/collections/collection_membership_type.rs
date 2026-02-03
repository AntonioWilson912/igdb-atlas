//! # Collection Membership Type Model
//!
//! Represents a collection membership type from the IGDB v4
//! `/collection_membership_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::collections::CollectionMembershipType;
//!
//! let json = r#"{
//!     "id": 50,
//!     "allowed_collection_type": 1,
//!     "name": "Member"
//! }"#;
//!
//! let collection_membership_type: CollectionMembershipType = serde_json::from_str(json).unwrap();
//! assert_eq!(collection_membership_type.name, Some("Member".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    collections::CollectionType,
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A collection membership type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMembershipType {
    /// Unique collection identifier.
    pub id: u64,

    /// The allowed collection type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub allowed_collection_type: Option<CollectionType>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Description of the membership type.
    #[serde(default)]
    pub description: Option<String>,

    /// The membership type name.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl CollectionMembershipType {}

impl Default for CollectionMembershipType {
    fn default() -> Self {
        Self {
            id: 0,
            allowed_collection_type: None,
            checksum: None,
            created_at: None,
            description: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for CollectionMembershipType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CollectionMembershipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CollectionMembershipType [{}]", self.id)?;
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
