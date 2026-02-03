//! # Platform Version Model
//!
//! Represents a platform version from the IGDB v4
//! `/platform_versions` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::platforms::PlatformVersion;
//!
//! let json = r#"{
//!     "id": 663,
//! 	"name": "Opera GX",
//! 	"platform_logo": 825,
//! 	"platform_version_release_dates": [760],
//! 	"slug": "opera-gx",
//! 	"url": "https://igdb.com/platforms/browser/version/opera-gx"
//! }"#;
//! let version: PlatformVersion = serde_json::from_str(json).unwrap();
//! assert_eq!(version.display_name(), "Opera GX");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    id_or_object::{FromId, deserialize_id_or_object, deserialize_id_or_object_vec},
    imagery::PlatformLogo,
    platforms::{PlatformVersionCompany, PlatformVersionReleaseDate},
};

/// A platform version release date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformVersion {
    /// Unique platform version release date identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Who developed this platform version.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub companies: Option<Vec<PlatformVersionCompany>>,

    /// The network capabilities
    #[serde(default)]
    pub connectivity: Option<String>,

    /// The integrated control processing unit
    #[serde(default)]
    pub cpu: Option<String>,

    /// The graphics chipset
    #[serde(default)]
    pub graphics: Option<String>,

    /// Who manufactured this version of the platform
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub main_manufacturer: Option<PlatformVersionCompany>,

    /// The type of media this version accepted
    #[serde(default)]
    pub media: Option<String>,

    /// How much memory there is
    #[serde(default)]
    pub memory: Option<String>,

    /// The name of the platform version
    #[serde(default)]
    pub name: Option<String>,

    /// The operating system installed on the platform version
    #[serde(default)]
    pub os: Option<String>,

    /// The output video rate
    #[serde(default)]
    pub output: Option<String>,

    /// The logo of this platform version
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub platform_logo: Option<PlatformLogo>,

    /// When this platform was released
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub platform_version_release_dates: Option<Vec<PlatformVersionReleaseDate>>,

    /// The maximum resolution
    #[serde(default)]
    pub resolutions: Option<String>,

    /// A url-safe, unique, lower-case version of the name
    #[serde(default)]
    pub slug: Option<String>,

    /// The sound chipset
    #[serde(default)]
    pub sound: Option<String>,

    /// How much storage there is
    #[serde(default)]
    pub storage: Option<String>,

    /// A short summary
    #[serde(default)]
    pub summary: Option<String>,

    /// The website address (URL) of the item
    #[serde(default)]
    pub url: Option<String>,
}

impl PlatformVersion {
    /// Returns the platform version or `"Unknown Platform Version"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Platform Version")
    }
}

impl Default for PlatformVersion {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            companies: None,
            connectivity: None,
            cpu: None,
            graphics: None,
            main_manufacturer: None,
            media: None,
            memory: None,
            name: None,
            os: None,
            output: None,
            platform_logo: None,
            platform_version_release_dates: None,
            resolutions: None,
            slug: None,
            sound: None,
            storage: None,
            summary: None,
            url: None,
        }
    }
}

impl FromId for PlatformVersion {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PlatformVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlatformVersion [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  {}", name)?;
        }
        if let Some(ref os) = self.os {
            writeln!(f, "  OS: {}", os)?;
        }
        if let Some(ref manufacturer) = self.main_manufacturer {
            writeln!(f, "  Manufacturer: {}", manufacturer)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
        }
        if let Some(ref summary) = self.summary {
            let truncated = if summary.len() > 200 {
                format!("{}...", &summary[..200])
            } else {
                summary.clone()
            };
            writeln!(f, "  Summary: {}", truncated)?;
        }
        if let Some(ref logo) = self.platform_logo {
            if let Some(ref img_id) = logo.image_id {
                writeln!(
                    f,
                    "  Cover: //images.igdb.com/igdb/image/upload/t_cover_big/{}.jpg",
                    img_id
                )?;
            }
        }
        writeln!(f)
    }
}
