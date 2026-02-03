//! # Platform Version Company Model
//!
//! Represents a platform developer from the IGDB v4
//! `/platform_version_companies` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::platforms::PlatformVersionCompany;
//!
//! let json = r#"{
//!     "id": 1,
//!     "company": 4,
//! 	"developer": true,
//!     "manufacturer": false
//! }"#;
//! let company: PlatformVersionCompany = serde_json::from_str(json).unwrap();
//! assert_eq!(company.developer, Some(true));
//! assert_eq!(company.manufacturer, Some(false));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Company,
    models::id_or_object::{FromId, deserialize_id_or_object},
};

/// A platform version company.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformVersionCompany {
    /// Unique platform version company identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Any notable comments about the developer.
    #[serde(default)]
    pub comment: Option<String>,

    /// The company responsible for developing this platform version.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub company: Option<Company>,

    /// Whether the company developed the platform version.
    #[serde(default)]
    pub developer: Option<bool>,

    /// Whether the company manufactured the platform version.
    #[serde(default)]
    pub manufacturer: Option<bool>,
}

impl Default for PlatformVersionCompany {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            comment: None,
            company: None,
            developer: None,
            manufacturer: None,
        }
    }
}

impl FromId for PlatformVersionCompany {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PlatformVersionCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlatformVersionCompany [{}]", self.id)?;
        if let Some(ref company) = self.company {
            write!(f, " Company ID: {}", company.id)?;
        }
        writeln!(f)
    }
}
