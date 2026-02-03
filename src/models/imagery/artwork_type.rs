//! # Artwork Type Model
//!
//! Represents an artwork image type from the IGDB v4
//! `/artwork_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::imagery::ArtworkType;
//!
//! let json = r#"{
//!     "id": 8,
//!     "name": "Infographic",
//!     "slug": "infographic"
//! }"#;
//!
//! let artwork_type: ArtworkType = serde_json::from_str(json).unwrap();
//! assert_eq!(artwork_type.name, Some("Infographic".to_string()));
//! assert_eq!(artwork_type.id, 8);
//! ```

use serde::{Deserialize, Serialize};

use crate::models::id_or_object::FromId;

/// An artwork type record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtworkType {
    /// Unique artwork type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The name of the artwork type.
    #[serde(default)]
    pub name: Option<String>,

    /// A url-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl ArtworkType {
    /// Returns the image ID or `"Unknown Artwork Type"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Artwork Type")
    }
}

impl Default for ArtworkType {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            slug: None,
            updated_at: None,
        }
    }
}

impl FromId for ArtworkType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for ArtworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ArtworkType [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        Ok(())
    }
}
