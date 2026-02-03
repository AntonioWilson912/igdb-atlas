//! # Alternative Name Model
//!
//! Represents an alternative or international game title from the
//! IGDB v4 `/alternative_names` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::AlternativeName;
//!
//! let json = r#"{
//!     "id": 5,
//!     "name": "TW3",
//!     "comment": "Community abbreviation",
//!     "game": 1942
//! }"#;
//!
//! let alt: AlternativeName = serde_json::from_str(json).unwrap();
//! assert_eq!(alt.display_name(), "TW3");
//! assert_eq!(alt.comment, Some("Community abbreviation".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::id_or_object::{FromId, deserialize_id_or_object},
};

/// An alternative or international title for a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeName {
    /// Unique alternative name identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// A description of what kind of alternative name it is
    /// (Acronym, Working title, Japanese title, etc.).
    #[serde(default)]
    pub comment: Option<String>,

    /// Reference ID to the game this alternative name belongs to.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// The alternative name string.
    #[serde(default)]
    pub name: Option<String>,
}

impl AlternativeName {
    /// Returns the alternative name or `"Unknown Name"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Name")
    }
}

impl Default for AlternativeName {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            comment: None,
            game: None,
            name: None,
        }
    }
}

impl FromId for AlternativeName {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for AlternativeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlternativeName [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " \"{}\"", name)?;
        }
        if let Some(ref comment) = self.comment {
            write!(f, " ({})", comment)?;
        }
        writeln!(f)
    }
}
