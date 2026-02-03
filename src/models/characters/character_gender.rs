//! # Character Gender Model
//!
//! Represents a character gender record from the IGDB v4
//! `/character_genders` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::characters::CharacterGenderRecord;
//!
//! let json = r#"{"id": 0, "name": "Male"}"#;
//! let record: CharacterGenderRecord = serde_json::from_str(json).unwrap();
//! assert_eq!(record.name, Some("Male".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A character-gender record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGenderRecord {
    /// Unique gender record identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Human-readable gender name.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl CharacterGenderRecord {
    /// Returns the gender name or `"Unknown Gender"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Gender")
    }
}

impl Default for CharacterGenderRecord {
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

impl FromId for CharacterGenderRecord {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CharacterGenderRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CharacterGender [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", date)?;
            }
        }
        writeln!(f)
    }
}
