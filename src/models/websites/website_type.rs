//! # Website Type Model
//!
//! Represents a website type from the IGDB v4 `/website_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::websites::WebsiteType;
//!
//! let json = r#"{"id": 13, "type": "Steam"}"#;
//! let wt: WebsiteType = serde_json::from_str(json).unwrap();
//! assert_eq!(wt.display_name(), "Steam");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A website type record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteType {
    /// Unique type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The website type name (e.g. "Steam", "Official", "Discord").
    #[serde(default)]
    pub r#type: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl WebsiteType {
    /// Returns the type name or `"Unknown Type"`.
    pub fn display_name(&self) -> &str {
        self.r#type.as_deref().unwrap_or("Unknown Type")
    }
}

impl Default for WebsiteType {
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

impl FromId for WebsiteType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for WebsiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebsiteType [{}]", self.id)?;
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
