//! # Search Model
//!
//! Represents a character, collection, company, game, platform, or theme search
//! from the IGDB v4 `/search` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::searches::Search;
//!
//! let json = r#"{
//!     "id": 98476,
//! 	"game": 1,
//!     "name": "Thief 2 The Metal Age",
//!     "published_at": 953596800
//! }"#;
//!
//! let search: Search = serde_json::from_str(json).unwrap();
//! assert_eq!(search.name, Some("Thief 2 The Metal Age".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Character, Collection, Company, Game, Platform, Theme,
    models::{
        id_or_object::{FromId, deserialize_id_or_object},
        timestamp::format_timestamp,
    },
};

/// A game search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search {
    /// Unique search identifier.
    pub id: u64,

    /// The alternative name of the object.
    #[serde(default)]
    pub alternative_name: Option<String>,

    /// The character associated with the object.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub character: Option<Character>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The collection associated with the object.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub collection: Option<Collection>,

    /// The company associated with the object.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub company: Option<Company>,

    /// The description of the object.
    #[serde(default)]
    pub description: Option<String>,

    /// The game associated with the object.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// The name of the object.
    #[serde(default)]
    pub name: Option<String>,

    /// The platform associated with the object.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub platform: Option<Platform>,

    /// Unix timestamp when this item was initially published by the third party.
    #[serde(default)]
    pub published_at: Option<i64>,

    /// The theme associated with the object.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub theme: Option<Theme>,
}

impl Search {
    /// Returns the object name or `"Unknown Object"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Object")
    }
}

impl Default for Search {
    fn default() -> Self {
        Self {
            id: 0,
            alternative_name: None,
            character: None,
            checksum: None,
            collection: None,
            company: None,
            description: None,
            game: None,
            name: None,
            platform: None,
            published_at: None,
            theme: None,
        }
    }
}

impl FromId for Search {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Search [{}]", self.id)?;
        if let Some(ref character) = self.character {
            writeln!(f, "  Character: {}", character)?;
        }
        if let Some(ref collection) = self.collection {
            writeln!(f, "  Collection: {}", collection)?;
        }
        if let Some(ref company) = self.company {
            writeln!(f, "  Company: {}", company)?;
        }
        if let Some(ref game) = self.game {
            writeln!(f, "  Game: {}", game)?;
        }
        if let Some(ref platform) = self.platform {
            writeln!(f, "  Platform: {}", platform)?;
        }
        if let Some(ref theme) = self.theme {
            writeln!(f, "  Theme: {}", theme)?;
        }
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ts) = self.published_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Published: {}", date)?;
            }
        }
        Ok(())
    }
}
