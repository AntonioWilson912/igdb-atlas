//! # Game Time to Beat Model
//!
//! Represents average time-to-beat data for a game from the IGDB v4
//! `/game_time_to_beats` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameTimeToBeat;
//!
//! let json = r#"{
//!     "id": 1,
//!     "game_id": 1942,
//!     "hastily": 93600,
//!     "normally": 180000,
//!     "completely": 432000,
//!     "count": 150
//! }"#;
//!
//! let ttb: GameTimeToBeat = serde_json::from_str(json).unwrap();
//! assert_eq!(ttb.hastily_hours(), Some(26.0));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        id_or_object::{FromId, deserialize_id_or_object},
        timestamp::format_timestamp,
    },
};

/// Average time-to-beat information for a game.
///
/// All time values are in seconds. Use the helper methods
/// for human-friendly hour conversions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTimeToBeat {
    /// Unique time-to-beat record identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Average time (in seconds) to 100% completion.
    #[serde(default)]
    pub completely: Option<u64>,

    /// Total number of time-to-beat submissions for this game.
    #[serde(default)]
    pub count: Option<u64>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The ID of the game associated with the time-to-beat data.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game_id: Option<Game>,

    /// Average time (in seconds) to finish without notable extras.
    #[serde(default)]
    pub hastily: Option<u64>,

    /// Average time (in seconds) while mixing in some extras.
    #[serde(default)]
    pub normally: Option<u64>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl GameTimeToBeat {
    /// Converts hastily (speed-run) seconds to hours.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::models::games::GameTimeToBeat;
    ///
    /// let ttb = GameTimeToBeat { id: 1, hastily: Some(5400), ..GameTimeToBeat::default() };
    /// assert_eq!(ttb.hastily_hours(), Some(1.5));
    /// ```
    pub fn hastily_hours(&self) -> Option<f64> {
        self.hastily.map(|s| s as f64 / 3600.0)
    }

    /// Converts normally (casual) seconds to hours.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::models::games::GameTimeToBeat;
    ///
    /// let ttb = GameTimeToBeat { id: 1, normally: Some(7200), ..GameTimeToBeat::default() };
    /// assert_eq!(ttb.normally_hours(), Some(2.0));
    /// ```
    pub fn normally_hours(&self) -> Option<f64> {
        self.normally.map(|s| s as f64 / 3600.0)
    }

    /// Converts completely (100%) seconds to hours.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::models::games::GameTimeToBeat;
    ///
    /// let ttb = GameTimeToBeat { id: 1, completely: Some(10800), ..GameTimeToBeat::default() };
    /// assert_eq!(ttb.completely_hours(), Some(3.0));
    /// ```
    pub fn completely_hours(&self) -> Option<f64> {
        self.completely.map(|s| s as f64 / 3600.0)
    }

    /// Formats a duration in seconds as `"Xh Ym"`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::models::games::GameTimeToBeat;
    ///
    /// assert_eq!(GameTimeToBeat::format_duration(Some(5400)), Some("1h 30m".to_string()));
    /// assert_eq!(GameTimeToBeat::format_duration(None), None);
    /// ```
    pub fn format_duration(seconds: Option<u64>) -> Option<String> {
        seconds.map(|s| {
            let hours = s / 3600;
            let minutes = (s % 3600) / 60;
            format!("{}h {}m", hours, minutes)
        })
    }
}

impl Default for GameTimeToBeat {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            completely: None,
            count: None,
            created_at: None,
            game_id: None,
            hastily: None,
            normally: None,
            updated_at: None,
        }
    }
}

impl FromId for GameTimeToBeat {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameTimeToBeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameTimeToBeat [{}]", self.id)?;
        if let Some(ref game_id) = self.game_id {
            writeln!(f, "  Game ID: {}", game_id)?;
        }
        if let Some(hastily) = self.hastily {
            writeln!(
                f,
                "  Hastily: {}",
                Self::format_duration(Some(hastily)).unwrap_or_default()
            )?;
        }
        if let Some(normally) = self.normally {
            writeln!(
                f,
                "  Normally: {}",
                Self::format_duration(Some(normally)).unwrap_or_default()
            )?;
        }
        if let Some(completely) = self.completely {
            writeln!(
                f,
                "  Completely: {}",
                Self::format_duration(Some(completely)).unwrap_or_default()
            )?;
        }
        if let Some(count) = self.count {
            writeln!(f, "  Submissions: {}", count)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Updated: {}", date)?;
            }
        }
        Ok(())
    }
}
