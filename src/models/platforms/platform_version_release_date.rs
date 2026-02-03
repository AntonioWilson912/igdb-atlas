//! # Platform Version Release Date Model
//!
//! Represents a platform release date from the IGDB v4
//! `/platform_version_release_dates` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::platforms::PlatformVersionReleaseDate;
//!
//! let json = r#"{
//!     "id": 572,
//!     "date": 1605139200,
//!     "y": 2020,
//!     "m": 11,
//!     "human": "Nov 12, 2020",
//!     "platform_version": 3
//! }"#;
//! let release_date: PlatformVersionReleaseDate = serde_json::from_str(json).unwrap();
//! assert_eq!(release_date.human, Some("Nov 12, 2020".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    DateFormat,
    models::{
        games::ReleaseDateRegion,
        id_or_object::{FromId, deserialize_id_or_object},
        platforms::PlatformVersion,
        timestamp::{format_timestamp, format_timestamp_pretty},
    },
};

/// A platform version release date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformVersionReleaseDate {
    /// Unique platform version release date identifier.
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

    /// Human-readable date string (e.g. `"Nov 19, 2015"`).
    #[serde(default)]
    pub human: Option<String>,

    /// Month as integer (1 = January).
    #[serde(default)]
    pub m: Option<u32>,

    /// The platform of the release.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub platform_version: Option<PlatformVersion>,

    /// The region of the release.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub release_region: Option<ReleaseDateRegion>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The year in full (e.g. 2015).
    #[serde(default)]
    pub y: Option<u32>,
}

impl PlatformVersionReleaseDate {
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

impl Default for PlatformVersionReleaseDate {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            date: None,
            date_format: None,
            human: None,
            m: None,
            platform_version: None,
            release_region: None,
            updated_at: None,
            y: None,
        }
    }
}

impl FromId for PlatformVersionReleaseDate {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PlatformVersionReleaseDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlatformVersionReleaseDate [{}]", self.id)?;
        write!(f, " {}", self.display_date())?;
        if let Some(ref platform_version) = self.platform_version {
            write!(f, " (platform version={})", platform_version)?;
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
