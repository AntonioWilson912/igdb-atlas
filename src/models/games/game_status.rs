//! # Game Status Model
//!
//! Represents a game release status from the IGDB v4 `/game_statuses` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameStatus;
//!
//! let json = r#"{"id": 0, "status": "Released"}"#;
//! let record: GameStatus = serde_json::from_str(json).unwrap();
//! assert_eq!(record.display_name(), "Released");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A game-status record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStatus {
    /// Unique status record identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Human-readable status name (e.g. `"Released"`, `"Alpha"`).
    #[serde(default)]
    pub status: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl GameStatus {
    /// Returns the status name or `"Unknown Status"`.
    pub fn display_name(&self) -> &str {
        self.status.as_deref().unwrap_or("Unknown Status")
    }
}

impl Default for GameStatus {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            status: None,
            updated_at: None,
        }
    }
}

impl FromId for GameStatus {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameStatus [{}]", self.id)?;
        if let Some(ref status) = self.status {
            write!(f, " {}", status)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", date)?;
            }
        }
        writeln!(f)
    }
}
