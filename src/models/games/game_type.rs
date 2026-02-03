//! # Game Type Model
//!
//! Represents a game type from the IGDB v4 `/game_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameType;
//!
//! let json = r#"{"id": 0, "type": "Main Game"}"#;
//! let game_type: GameType = serde_json::from_str(json).unwrap();
//! assert_eq!(game_type.r#type, Some("Main Game".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A game type record (e.g. "Main Game", "DLC", "Expansion").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameType {
    /// Unique type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The type name (e.g. "Main Game", "DLC", "Expansion").
    #[serde(default)]
    pub r#type: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl GameType {
    /// Returns the type name or `"Unknown Type"`.
    pub fn display_name(&self) -> &str {
        self.r#type.as_deref().unwrap_or("Unknown Type")
    }
}

impl Default for GameType {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            r#type: None,
            updated_at: None,
        }
    }
}

impl FromId for GameType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameType [{}]", self.id)?;
        if let Some(ref t) = self.r#type {
            write!(f, " {}", t)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", date)?;
            }
        }
        writeln!(f)
    }
}
