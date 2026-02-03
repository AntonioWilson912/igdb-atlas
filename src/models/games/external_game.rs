//! # External Game Model
//!
//! Represents a game listing on an external service from the IGDB v4
//! `/external_games` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::ExternalGame;
//!
//! let json = r#"{
//!     "id": 100,
//!     "uid": "292030",
//!     "name": "The Witcher 3: Wild Hunt",
//!     "external_game_source": 1,
//!     "url": "https://store.steampowered.com/app/292030",
//!     "game": 1942,
//!     "year": 2015
//! }"#;
//!
//! let ext: ExternalGame = serde_json::from_str(json).unwrap();
//! assert_eq!(ext.uid, Some("292030".to_string()));
//! assert_eq!(ext.year, Some(2015));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Platform,
    models::{
        games::{ExternalGameSource, Game, GameReleaseFormat},
        id_or_object::{FromId, deserialize_id_or_object},
        timestamp::format_timestamp,
    },
};

/// A game listing on an external service (Steam, GOG, Epic, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalGame {
    /// Unique external game identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// ISO country codes where this external listing is available.
    #[serde(default)]
    pub countries: Option<Vec<u32>>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Reference ID to the `/external_game_sources` endpoint.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub external_game_source: Option<ExternalGameSource>,

    /// Reference ID to the IGDB game.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Reference ID to the `/game_release_formats` endpoint.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game_release_format: Option<GameReleaseFormat>,

    /// The name of the game on the external service.
    #[serde(default)]
    pub name: Option<String>,

    /// Reference ID to the platform.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub platform: Option<Platform>,

    /// The external service's unique identifier for this game.
    #[serde(default)]
    pub uid: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// URL to the game on the external service.
    #[serde(default)]
    pub url: Option<String>,

    /// The release year (e.g. 2015).
    #[serde(default)]
    pub year: Option<u32>,
}

impl ExternalGame {
    /// Returns the external game name or `"Unknown External Game"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown External Game")
    }
}

impl Default for ExternalGame {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            countries: None,
            created_at: None,
            external_game_source: None,
            game: None,
            game_release_format: None,
            name: None,
            platform: None,
            uid: None,
            updated_at: None,
            url: None,
            year: None,
        }
    }
}

impl FromId for ExternalGame {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ExternalGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ExternalGame [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref uid) = self.uid {
            writeln!(f, "  UID: {}", uid)?;
        }
        if let Some(ref source) = self.external_game_source {
            writeln!(f, "  Source ID: {}", source)?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
        }
        if let Some(year) = self.year {
            writeln!(f, "  Year: {}", year)?;
        }
        if let Some(ref game) = self.game {
            writeln!(f, "  Game ID: {}", game)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Updated: {}", date)?;
            }
        }
        Ok(())
    }
}
