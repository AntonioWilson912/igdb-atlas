//! # Language Model
//!
//! Represents a language definition from the IGDB v4 `/languages` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::languages::Language;
//!
//! let json = r#"{"id": 5, "name": "English", "locale": "en", "native_name": "English"}"#;
//! let lang: Language = serde_json::from_str(json).unwrap();
//! assert_eq!(lang.display_name(), "English");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A language entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    /// Unique language identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// BCP 47 locale code (e.g. `"en"`, `"ja"`).
    #[serde(default)]
    pub locale: Option<String>,

    /// English name of the language.
    #[serde(default)]
    pub name: Option<String>,

    /// Name of the language written in that language itself.
    #[serde(default)]
    pub native_name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Language {
    /// Returns the language name or `"Unknown Language"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Language")
    }
}

impl Default for Language {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            locale: None,
            name: None,
            native_name: None,
            updated_at: None,
        }
    }
}

impl FromId for Language {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Language [{}]", self.id)?;
        if let Some(ref locale) = self.locale {
            write!(f, " locale={}", locale)?;
        }
        if let Some(ref name) = self.name {
            write!(f, " name={}", name)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", d)?;
            }
        }
        writeln!(f)
    }
}
