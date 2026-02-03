//! # Game Release Format Model
//!
//! Represents a game release format from the IGDB v4
//! `/game_release_formats` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameReleaseFormat;
//!
//! let json = r#"{"id": 1, "format": "Physical"}"#;
//! let format: GameReleaseFormat = serde_json::from_str(json).unwrap();
//! assert_eq!(format.format, Some("Physical".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A game release format record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameReleaseFormat {
    /// Unique format identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The format name (e.g. "Physical", "Digital").
    #[serde(default)]
    pub format: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl GameReleaseFormat {
    /// Returns the format name or `"Unknown Format"`.
    pub fn display_name(&self) -> &str {
        self.format.as_deref().unwrap_or("Unknown Format")
    }
}

impl Default for GameReleaseFormat {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            format: None,
            updated_at: None,
        }
    }
}

impl FromId for GameReleaseFormat {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameReleaseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameReleaseFormat [{}]", self.id)?;
        if let Some(ref format) = self.format {
            write!(f, " {}", format)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", date)?;
            }
        }
        writeln!(f)
    }
}
