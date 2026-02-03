//! # Theme Model
//!
//! Represents a game theme from the IGDB v4 `/themes` endpoint.
//!
//! Themes describe narrative or atmospheric elements (Fantasy,
//! Science Fiction, Horror) as opposed to genres which describe gameplay.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::themes::Theme;
//!
//! let json = r#"{
//!     "id": 18,
//!     "name": "Fantasy",
//!     "slug": "fantasy"
//! }"#;
//!
//! let theme: Theme = serde_json::from_str(json).unwrap();
//! assert_eq!(theme.name, Some("Fantasy".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A game theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Unique theme identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Theme name.
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

impl Theme {
    /// Returns the theme name or `"Unknown Theme"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Theme")
    }
}

impl Default for Theme {
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

impl FromId for Theme {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Theme [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
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
