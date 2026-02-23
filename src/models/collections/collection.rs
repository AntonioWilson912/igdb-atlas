//! # Collection Model
//!
//! Represents a collection or series of games from the IGDB v4
//! `/collections` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::collections::Collection;
//!
//! let json = r#"{
//!     "id": 50,
//!     "name": "The Legend of Zelda",
//!     "games": [1, 2, 3, 4, 5],
//!     "slug": "the-legend-of-zelda"
//! }"#;
//!
//! let collection: Collection = serde_json::from_str(json).unwrap();
//! assert_eq!(collection.name, Some("The Legend of Zelda".to_string()));
//! assert_eq!(collection.game_count(), 5);
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        collections::{CollectionRelation, CollectionType},
        id_or_object::{FromId, deserialize_id_or_object_vec},
        timestamp::format_timestamp,
    },
};

/// A collection or series of games.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    /// Unique collection identifier.
    pub id: u64,

    /// Collection relation IDs where this collection is a child.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub as_child_relations: Option<Vec<CollectionRelation>>,

    /// Collection relation IDs where this collection is a parent.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub as_parent_relations: Option<Vec<CollectionRelation>>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Game IDs associated with this collection.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub games: Option<Vec<Game>>,

    /// Collection name.
    #[serde(default)]
    pub name: Option<String>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// The type of collection.
    #[serde(default)]
    pub r#type: Option<CollectionType>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The IGDB page URL.
    #[serde(default)]
    pub url: Option<String>,
}

impl Collection {
    /// Returns the collection name or `"Unknown Collection"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Collection")
    }

    /// Returns the number of games in this collection.
    pub fn game_count(&self) -> usize {
        self.games.as_ref().map(|g| g.len()).unwrap_or(0)
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            id: 0,
            as_child_relations: None,
            as_parent_relations: None,
            checksum: None,
            created_at: None,
            games: None,
            name: None,
            slug: None,
            r#type: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for Collection {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Collection [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref games) = self.games {
            writeln!(f, "  Games: {}", games.len())?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
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
