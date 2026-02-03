//! # Platform Family Model
//!
//! Represents a family of related platforms from the IGDB v4
//! `/platform_families` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::platforms::PlatformFamily;
//!
//! let json = r#"{"id": 7, "name": "PlayStation"}"#;
//! let family: PlatformFamily = serde_json::from_str(json).unwrap();
//! assert_eq!(family.display_name(), "PlayStation");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::id_or_object::FromId;

/// A family or group of related platforms (e.g. all PlayStation
/// consoles, all Xbox consoles).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlatformFamily {
    /// Unique platform family identifier.
    pub id: u64,

    /// Family name (e.g. "PlayStation", "Xbox").
    #[serde(default)]
    pub name: Option<String>,
}

impl PlatformFamily {
    /// Returns the family name or `"Unknown Family"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Family")
    }
}

impl FromId for PlatformFamily {
    fn from_id(id: u64) -> Self {
        Self { id, name: None }
    }
}

impl std::fmt::Display for PlatformFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlatformFamily [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        writeln!(f)
    }
}
