//! # Genre Model
//!
//! Represents a video game genre from the IGDB v4 `/genres` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::Genre;
//!
//! let json = r#"{
//!     "id": 12,
//!     "name": "Role-playing (RPG)",
//!     "slug": "role-playing-rpg"
//! }"#;
//!
//! let genre: Genre = serde_json::from_str(json).unwrap();
//! assert_eq!(genre.display_name(), "Role-playing (RPG)");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A video game genre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    /// Unique genre identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Genre name.
    #[serde(default)]
    pub name: Option<String>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The IGDB page URL.
    #[serde(default)]
    pub url: Option<String>,
}

impl Genre {
    /// Returns the genre name or `"Unknown Genre"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Genre")
    }
}

impl Default for Genre {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            slug: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for Genre {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Genre [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            write!(f, " ({})", slug)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " updated {}", date)?;
            }
        }
        writeln!(f)
    }
}
