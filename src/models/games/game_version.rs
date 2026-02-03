//! # Game Version Model
//!
//! Represents game edition and version details from the IGDB v4
//! `/game_versions` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameVersion;
//!
//! let json = r#"{
//!     "id": 1,
//!     "game": 1942,
//!     "games": [1942, 1943, 1944],
//!     "features": [1, 2, 3]
//! }"#;
//!
//! let version: GameVersion = serde_json::from_str(json).unwrap();
//! assert_eq!(version.game.as_ref().map(|o| o.id), Some(1942));
//! assert_eq!(version.edition_count(), 3);
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::{Game, GameVersionFeature},
    id_or_object::{FromId, deserialize_id_or_object, deserialize_id_or_object_vec},
    timestamp::format_timestamp,
};

/// Details about game editions and versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVersion {
    /// Unique version record identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Feature IDs describing what makes each version different.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub features: Option<Vec<GameVersionFeature>>,

    /// Reference ID to the main game these versions are of.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Game IDs for the versions and editions.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub games: Option<Vec<Game>>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The website address (URL) of the item.
    #[serde(default)]
    pub url: Option<String>,
}

impl GameVersion {
    /// Returns the number of games/editions in this version set.
    pub fn edition_count(&self) -> usize {
        self.games.as_ref().map(|g| g.len()).unwrap_or(0)
    }
}

impl Default for GameVersion {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            features: None,
            game: None,
            games: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for GameVersion {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameVersion [{}]", self.id)?;
        if let Some(ref game) = self.game {
            writeln!(f, "  Main Game ID: {}", game)?;
        }
        if let Some(ref games) = self.games {
            writeln!(f, "  Editions: {} games", games.len())?;
        }
        if let Some(ref features) = self.features {
            writeln!(f, "  Features: {} defined", features.len())?;
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
