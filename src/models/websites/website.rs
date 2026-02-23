//! # Website Model
//!
//! Represents a URL associated with a game from the IGDB v4
//! `/websites` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::websites::Website;
//!
//! let json = r#"{
//!     "id": 100,
//!     "url": "https://store.steampowered.com/app/292030",
//!     "type": 13,
//!     "game": 1942,
//!     "trusted": true
//! }"#;
//!
//! let site: Website = serde_json::from_str(json).unwrap();
//! assert_eq!(site.url, Some("https://store.steampowered.com/app/292030".to_string()));
//! assert_eq!(site.trusted, Some(true));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        id_or_object::{FromId, deserialize_id_or_object},
        websites::WebsiteType,
    },
};

/// A website URL associated with a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Website {
    /// Unique website identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The game this website is associated with.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Whether this website is a trusted / verified source.
    #[serde(default)]
    pub trusted: Option<bool>,

    /// The website type associated with the website.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub r#type: Option<WebsiteType>,

    /// The website address (URL).
    #[serde(default)]
    pub url: Option<String>,
}

impl Website {
    /// Returns the URL or `"No URL"`.
    pub fn display_url(&self) -> &str {
        self.url.as_deref().unwrap_or("No URL")
    }

    /// Extracts the domain name from the URL.
    pub fn domain(&self) -> Option<&str> {
        self.url.as_deref().and_then(|url| {
            url.strip_prefix("https://")
                .or_else(|| url.strip_prefix("http://"))
                .and_then(|rest| rest.split('/').next())
        })
    }
}

impl Default for Website {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            game: None,
            trusted: None,
            r#type: None,
            url: None,
        }
    }
}

impl FromId for Website {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Website {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Website [{}]", self.id)?;
        if let Some(ref url) = self.url {
            write!(f, " {}", url)?;
        }
        if let Some(ref r#type) = self.r#type {
            write!(f, " (type={})", r#type)?;
        }
        if self.trusted == Some(true) {
            write!(f, " [trusted]")?;
        }
        writeln!(f)
    }
}
