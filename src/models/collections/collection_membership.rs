//! # Collection Membership Model
//!
//! Represents a collection membership from the IGDB v4
//! `/collection_memberships` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::collections::CollectionMembership;
//!
//! let json = r#"{
//!     "id": 50,
//!     "collection": 1,
//!     "game": 2
//! }"#;
//!
//! let collection_membership: CollectionMembership = serde_json::from_str(json).unwrap();
//! assert_eq!(collection_membership.collection.as_ref().map(|o| o.id), Some(1));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        collections::{Collection, CollectionMembershipType},
        id_or_object::{FromId, deserialize_id_or_object},
        timestamp::format_timestamp,
    },
};

/// A collection membership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMembership {
    /// Unique collection identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The collection that is associated with this membership.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub collection: Option<Collection>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The game that is associated with this membership.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// The type of collection membership.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub r#type: Option<CollectionMembershipType>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl CollectionMembership {}

impl Default for CollectionMembership {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            collection: None,
            created_at: None,
            game: None,
            r#type: None,
            updated_at: None,
        }
    }
}

impl FromId for CollectionMembership {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CollectionMembership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CollectionMembership [{}]", self.id)?;
        if let Some(ref collection) = self.collection {
            writeln!(f, "  Collection: {}", collection)?;
        }
        if let Some(ref game) = self.game {
            writeln!(f, "  Game: {}", game)?;
        }
        if let Some(ref t) = self.r#type {
            writeln!(f, "  Membership Type: {}", t)?;
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
