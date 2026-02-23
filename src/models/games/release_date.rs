//! # Release Date Model
//!
//! Represents a game release date from the IGDB v4 `/release_dates` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::ReleaseDate;
//!
//! let json = r#"{
//!     "id": 1,
//!     "date": 1448150400,
//!     "y": 2015,
//!     "m": 11,
//!     "human": "Nov 19, 2015",
//!     "platform": 48,
//!     "game": 1942
//! }"#;
//!
//! let rd: ReleaseDate = serde_json::from_str(json).unwrap();
//! assert_eq!(rd.y, Some(2015));
//! assert_eq!(rd.human, Some("Nov 19, 2015".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    DateFormat, Game, Platform,
    models::{
        games::{ReleaseDateRegion, ReleaseDateStatus},
        id_or_object::{FromId, deserialize_id_or_object},
        timestamp::{format_timestamp, format_timestamp_pretty},
    },
};

/// A game release date entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDate {
    /// Unique release date identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Unix timestamp of the release date.
    #[serde(default)]
    pub date: Option<i64>,

    /// The format of the change date.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub date_format: Option<DateFormat>,

    /// The game of the release.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Human-readable date string (e.g. `"Nov 19, 2015"`).
    #[serde(default)]
    pub human: Option<String>,

    /// Month as integer (1 = January).
    #[serde(default)]
    pub m: Option<u32>,

    /// The platform of the release.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub platform: Option<Platform>,

    /// The region of the release.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub release_region: Option<ReleaseDateRegion>,

    /// The status of the release.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub status: Option<ReleaseDateStatus>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The year in full (e.g. 2015).
    #[serde(default)]
    pub y: Option<u32>,
}

impl ReleaseDate {
    /// Returns the human-readable date, falling back to formatted
    /// timestamp, year-month, year, or `"Unknown Date"`.
    pub fn display_date(&self) -> String {
        if let Some(ref human) = self.human {
            return human.clone();
        }
        if let Some(pretty) = format_timestamp_pretty(self.date) {
            return pretty;
        }
        if let (Some(y), Some(m)) = (self.y, self.m) {
            return format!("{:04}-{:02}", y, m);
        }
        if let Some(y) = self.y {
            return format!("{}", y);
        }
        "Unknown Date".to_string()
    }
}

impl Default for ReleaseDate {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            date: None,
            date_format: None,
            game: None,
            human: None,
            m: None,
            platform: None,
            release_region: None,
            status: None,
            updated_at: None,
            y: None,
        }
    }
}

impl FromId for ReleaseDate {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ReleaseDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReleaseDate [{}]", self.id)?;
        write!(f, " {}", self.display_date())?;
        if let Some(ref platform) = self.platform {
            write!(f, " (platform={})", platform)?;
        }
        if let Some(ref region) = self.release_region {
            write!(f, " region={}", region)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " updated {}", date)?;
            }
        }
        writeln!(f)
    }
}
