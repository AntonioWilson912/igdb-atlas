//! # Game Localization Model
//!
//! Represents localized information for a game from the IGDB v4
//! `/game_localizations` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameLocalization;
//!
//! let json = r#"{
//!     "id": 100,
//!     "name": "ウィッチャー3",
//!     "game": 1942,
//!     "region": 5
//! }"#;
//!
//! let loc: GameLocalization = serde_json::from_str(json).unwrap();
//! assert_eq!(loc.name, Some("ウィッチャー3".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::{Game, Region},
    id_or_object::{FromId, deserialize_id_or_object},
    imagery::Cover,
    timestamp::format_timestamp,
};

/// Localized information for a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameLocalization {
    /// Unique localization record identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// T cover image for this localization.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub cover: Option<Cover>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The game the localization belongs to.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Localized game name.
    #[serde(default)]
    pub name: Option<String>,

    /// The region of the localization.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub region: Option<Region>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl GameLocalization {
    /// Returns the localized name or `"Unknown"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown")
    }
}

impl Default for GameLocalization {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            cover: None,
            created_at: None,
            game: None,
            name: None,
            region: None,
            updated_at: None,
        }
    }
}

impl FromId for GameLocalization {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameLocalization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameLocalization [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref game) = self.game {
            writeln!(f, "  Game ID: {}", game.id)?;
        }
        if let Some(ref region) = self.region {
            writeln!(f, "  Region ID: {}", region.id)?;
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
