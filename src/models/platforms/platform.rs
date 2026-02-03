//! # Platform Model
//!
//! Represents a gaming platform from the IGDB v4 `/platforms` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::platforms::Platform;
//!
//! let json = r#"{
//!     "id": 48,
//!     "name": "PlayStation 4",
//!     "abbreviation": "PS4",
//!     "platform_family": {"id": 7, "name": "PlayStation"},
//!     "generation": 8,
//!     "slug": "playstation-4"
//! }"#;
//!
//! let platform: Platform = serde_json::from_str(json).unwrap();
//! assert_eq!(platform.name, Some("PlayStation 4".to_string()));
//! assert_eq!(platform.abbreviation, Some("PS4".to_string()));
//! assert_eq!(platform.generation, Some(8));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    id_or_object::{FromId, deserialize_id_or_object},
    imagery::{PlatformLogo, UrlFor},
    platforms::PlatformFamily,
    timestamp::format_timestamp,
};

/// A gaming platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platform {
    /// Unique platform identifier.
    pub id: u64,

    /// Short abbreviation (e.g. `"PS4"`, `"XB1"`).
    #[serde(default)]
    pub abbreviation: Option<String>,

    /// An alternative name for this platform.
    #[serde(default)]
    pub alternative_name: Option<String>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Console generation number (e.g. 8 for PS4 / Xbox One).
    #[serde(default)]
    pub generation: Option<u16>,

    /// The platform's display name.
    #[serde(default)]
    pub name: Option<String>,

    /// The platform family this platform belongs to.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub platform_family: Option<PlatformFamily>,

    /// The platform's logo image.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub platform_logo: Option<PlatformLogo>,

    /// Reference ID to the `/platform_types` endpoint.
    #[serde(default)]
    pub platform_type: Option<u64>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Summary / description of the platform.
    #[serde(default)]
    pub summary: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The IGDB page URL.
    #[serde(default)]
    pub url: Option<String>,

    /// Associated platform version IDs.
    #[serde(default)]
    pub versions: Option<Vec<u64>>,

    /// Website IDs associated with this platform.
    #[serde(default)]
    pub websites: Option<Vec<u64>>,
}

impl Platform {
    /// Returns the platform name, falling back to abbreviation, then
    /// `"Unknown Platform"`.
    pub fn display_name(&self) -> &str {
        self.name
            .as_deref()
            .or(self.abbreviation.as_deref())
            .unwrap_or("Unknown Platform")
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            id: 0,
            abbreviation: None,
            alternative_name: None,
            checksum: None,
            created_at: None,
            generation: None,
            name: None,
            platform_family: None,
            platform_logo: None,
            platform_type: None,
            slug: None,
            summary: None,
            updated_at: None,
            url: None,
            versions: None,
            websites: None,
        }
    }
}

impl FromId for Platform {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Platform [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref abbr) = self.abbreviation {
            writeln!(f, "  Abbreviation: {}", abbr)?;
        }
        if let Some(ref alt) = self.alternative_name {
            writeln!(f, "  Alternative Name: {}", alt)?;
        }
        if let Some(r#gen) = self.generation {
            writeln!(f, "  Generation: {}", r#gen)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref summary) = self.summary {
            writeln!(f, "  Summary: {}", summary)?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
        }
        if let Some(ref family) = self.platform_family {
            if let Some(ref name) = family.name {
                writeln!(f, "  Family: {}", name)?;
            } else {
                writeln!(f, "  Family: ID {}", family.id)?;
            }
        }
        if let Some(ref logo) = self.platform_logo {
            if let Some(url) = logo.url("logo_small") {
                writeln!(f, "  Logo: {}", url)?;
            } else {
                writeln!(f, "  Logo: ID {}", logo.id)?;
            }
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
