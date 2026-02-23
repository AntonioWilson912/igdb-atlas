//! # Language Support Model
//!
//! Represents a per-game language support record from the IGDB v4
//! `/language_supports` endpoint.
//!
//! Each entry links a game, a language, and a support type
//! (audio, subtitles, or interface).
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::languages::LanguageSupport;
//!
//! let json = r#"{
//!     "id": 1,
//!     "game": 1942,
//!     "language": 5,
//!     "language_support_type": 1
//! }"#;
//!
//! let ls: LanguageSupport = serde_json::from_str(json).unwrap();
//! assert_eq!(ls.game.as_ref().map(|o| o.id), Some(1942));
//! assert_eq!(ls.language.as_ref().map(|o| o.id), Some(5));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Language,
    models::{
        common::GameRef,
        id_or_object::{FromId, deserialize_id_or_object},
        languages::LanguageSupportType,
        timestamp::format_timestamp,
    },
};

/// A language support entry linking a game with a language and
/// the type of support provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSupport {
    /// Unique language support identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The game.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<GameRef>,

    /// The language.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub language: Option<Language>,

    /// The type of language support.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub language_support_type: Option<LanguageSupportType>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Default for LanguageSupport {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            game: None,
            language: None,
            language_support_type: None,
            updated_at: None,
        }
    }
}

impl FromId for LanguageSupport {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for LanguageSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LanguageSupport [{}]", self.id)?;
        if let Some(ref lang) = self.language {
            write!(f, " lang={}", lang)?;
        }
        if let Some(ref lst) = self.language_support_type {
            write!(f, " type={}", lst)?;
        }
        if let Some(ref game) = self.game {
            write!(f, " game={}", game.id)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", d)?;
            }
        }
        writeln!(f)
    }
}
