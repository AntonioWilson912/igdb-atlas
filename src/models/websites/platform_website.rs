//! # Platform Website Model
//!
//! Represents a platform website from the IGDB v4 `/platform_websites` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::websites::PlatformWebsite;
//!
//! let json = r#"{
//!     "id": 205,
//!     "url": "https://en.wikipedia.org/wiki/Satellaview",
//!     "type": 1,
//!     "trusted": false
//! }"#;
//! let cw: PlatformWebsite = serde_json::from_str(json).unwrap();
//! assert_eq!(cw.url, Some("https://en.wikipedia.org/wiki/Satellaview".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    id_or_object::{FromId, deserialize_id_or_object},
    websites::WebsiteType,
};

/// A platform website record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformWebsite {
    /// Unique platform website identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Whether the website is trusted.
    #[serde(default)]
    pub trusted: Option<bool>,

    /// The website type associated with the website.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub r#type: Option<WebsiteType>,

    /// The website address (URL) of the item.
    #[serde(default)]
    pub url: Option<String>,
}

impl Default for PlatformWebsite {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            trusted: None,
            r#type: None,
            url: None,
        }
    }
}

impl FromId for PlatformWebsite {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PlatformWebsite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlatformWebsite [{}]", self.id)?;
        if let Some(ref url) = self.url {
            write!(f, " {}", url)?;
        }
        if let Some(ref type_id) = self.r#type {
            write!(f, " (type={})", type_id)?;
        }
        if self.trusted == Some(true) {
            write!(f, " [trusted]")?;
        }
        writeln!(f)
    }
}
