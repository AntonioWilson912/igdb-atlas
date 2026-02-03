//! # Date Format Model
//!
//! Represents a date format from the IGDB v4 `/date_formats` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::date_formats::DateFormat;
//!
//! let json = r#"{
//!     "id": 0,
//!     "format": "YYYYMMDD"
//! }"#;
//!
//! let date_format: DateFormat = serde_json::from_str(json).unwrap();
//! assert_eq!(date_format.format, Some("YYYYMMDD".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A date format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateFormat {
    /// Unique date format identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Format of the date.
    #[serde(default)]
    pub format: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl DateFormat {
    /// Returns the format or `"Unknown Date Format"`.
    pub fn display_name(&self) -> &str {
        self.format.as_deref().unwrap_or("Unknown Date Format")
    }
}

impl Default for DateFormat {
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

impl FromId for DateFormat {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for DateFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DateFormat [{}]", self.id)?;
        if let Some(ref format) = self.format {
            writeln!(f, "  Format: {}", format)?;
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
