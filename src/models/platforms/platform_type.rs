//! # Platform Type Model
//!
//! Represents a platform type (console, computer, etc.) from the IGDB v4
//! `/platform_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::platforms::PlatformType;
//!
//! let json = r#"{"id": 2, "name": "Arcade"}"#;
//! let platform_type: PlatformType = serde_json::from_str(json).unwrap();
//! assert_eq!(platform_type.display_name(), "Arcade");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::id_or_object::FromId;

/// A platform type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformType {
    /// Unique platform type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Family name (e.g. "PlayStation", "Xbox").
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl PlatformType {
    /// Returns the platform type or `"Unknown Platform Type"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Platform Type")
    }
}

impl Default for PlatformType {
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

impl FromId for PlatformType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PlatformType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlatformType [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        writeln!(f)
    }
}
