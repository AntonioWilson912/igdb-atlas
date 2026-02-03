//! # Company Website Model
//!
//! Represents a company website from the IGDB v4 `/company_websites` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::websites::CompanyWebsite;
//!
//! let json = r#"{
//!     "id": 443,
//!     "url": "https://www.digitalscapes.ca/",
//!     "type": 1,
//!     "trusted": false
//! }"#;
//! let cw: CompanyWebsite = serde_json::from_str(json).unwrap();
//! assert_eq!(cw.url, Some("https://www.digitalscapes.ca/".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    id_or_object::{FromId, deserialize_id_or_object},
    websites::WebsiteType,
};

/// A company website record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyWebsite {
    /// Unique company website identifier.
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

impl Default for CompanyWebsite {
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

impl FromId for CompanyWebsite {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CompanyWebsite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompanyWebsite [{}]", self.id)?;
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
