//! # Language Support Type Model
//!
//! Represents a language support type from the IGDB v4
//! `/language_support_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::languages::LanguageSupportType;
//!
//! let json = r#"{"id": 1, "name": "Audio"}"#;
//! let lst: LanguageSupportType = serde_json::from_str(json).unwrap();
//! assert_eq!(lst.display_name(), "Audio");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A language support type record (e.g. Audio, Subtitles, Interface).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSupportType {
    /// Unique type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Support type name (e.g. `"Audio"`, `"Subtitles"`, `"Interface"`).
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl LanguageSupportType {
    /// Returns the type name or `"Unknown Type"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Type")
    }
}

impl Default for LanguageSupportType {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for LanguageSupportType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for LanguageSupportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LanguageSupportType [{}]", self.id)?;
        if let Some(ref n) = self.name {
            write!(f, " {}", n)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", d)?;
            }
        }
        writeln!(f)
    }
}
