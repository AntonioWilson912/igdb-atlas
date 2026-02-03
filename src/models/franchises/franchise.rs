//! # Franchise Model
//!
//! Represents a franchise from the IGDB v4 `/franchises` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::franchises::Franchise;
//!
//! let json = r#"{
//!     "id": 4289,
//!     "name": "Space Dandy",
//!     "url": "https://www.igdb.com/franchises/space-dandy"
//! }"#;
//!
//! let franchise: Franchise = serde_json::from_str(json).unwrap();
//! assert_eq!(franchise.name, Some("Space Dandy".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        id_or_object::{FromId, deserialize_id_or_object_vec},
        timestamp::format_timestamp,
    },
};

/// A video game franchise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Franchise {
    /// Unique franchise identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Games that are associated with this franchise.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub games: Option<Vec<Game>>,

    /// The name of the franchise.
    #[serde(default)]
    pub name: Option<String>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The website address (URL) of the item.
    #[serde(default)]
    pub url: Option<String>,
}

impl Franchise {
    /// Returns the franchise name or `"Unknown Franchise"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Franchise")
    }
}

impl Default for Franchise {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            games: None,
            name: None,
            slug: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for Franchise {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Franchise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Franchise [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  Livestream URL: {}", url)?;
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
