//! # Game Mode Model
//!
//! Represents a play mode available for a game from the IGDB v4
//! `/game_modes` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameMode;
//!
//! let json = r#"{"id": 1, "name": "Single player", "slug": "single-player"}"#;
//! let mode: GameMode = serde_json::from_str(json).unwrap();
//! assert_eq!(mode.name, Some("Single player".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A game mode (e.g. single player, multiplayer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMode {
    /// Unique game mode identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The name of the game mode.
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

impl GameMode {
    /// Returns the mode name or `"Unknown Mode"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Mode")
    }

    /// Returns `true` if this mode indicates single-player support (ID 1).
    pub fn is_single_player(&self) -> bool {
        self.id == 1
    }

    /// Returns `true` if this mode indicates multiplayer support
    /// (IDs 2–6: MP, Co-op, Split screen, MMO, Battle Royale).
    pub fn is_multiplayer(&self) -> bool {
        matches!(self.id, 2 | 3 | 4 | 5 | 6)
    }
}

impl Default for GameMode {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            slug: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for GameMode {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameMode [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
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
