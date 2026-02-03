//! # Release Date Status Model
//!
//! Represents a release date status from the IGDB v4
//! `/release_date_statuses` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::ReleaseDateStatus;
//!
//! let json = r#"{"id": 1, "name": "Released", "description": "The game has been released."}"#;
//! let status: ReleaseDateStatus = serde_json::from_str(json).unwrap();
//! assert_eq!(status.display_name(), "Released");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A release date status record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDateStatus {
    /// Unique status identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Description of the status.
    #[serde(default)]
    pub description: Option<String>,

    /// Status name (e.g. "Released", "Alpha", "TBD").
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl ReleaseDateStatus {
    /// Returns the status name or `"Unknown Status"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Status")
    }
}

impl Default for ReleaseDateStatus {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            description: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for ReleaseDateStatus {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ReleaseDateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReleaseDateStatus [{}]", self.id)?;
        if let Some(ref n) = self.name {
            write!(f, " {}", n)?;
        }
        if let Some(ref d) = self.description {
            write!(f, " - {}", d)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", d)?;
            }
        }
        writeln!(f)
    }
}
