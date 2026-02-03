//! # Platform Logo Model
//!
//! Represents a platform logo from the IGDB v4 `/platform_logos` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::imagery::PlatformLogo;
//!
//! let json = r#"{
//!     "id": 1,
//!     "image_id": "platform_abc123",
//!     "width": 200,
//!     "height": 100
//! }"#;
//!
//! let logo: PlatformLogo = serde_json::from_str(json).unwrap();
//! assert_eq!(logo.image_id, Some("platform_abc123".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::id_or_object::FromId;

/// A platform logo image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformLogo {
    /// Unique logo identifier.
    pub id: u64,

    /// Whether the image has an alpha channel (transparency).
    #[serde(default)]
    pub alpha_channel: Option<bool>,

    /// Whether the image is animated (e.g. GIF).
    #[serde(default)]
    pub animated: Option<bool>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Image height in pixels.
    #[serde(default)]
    pub height: Option<u32>,

    /// The image ID used to construct IGDB image URLs.
    #[serde(default)]
    pub image_id: Option<String>,

    /// Direct URL to the image as stored by IGDB.
    #[serde(default)]
    pub url: Option<String>,

    /// Image width in pixels.
    #[serde(default)]
    pub width: Option<u32>,
}

impl Default for PlatformLogo {
    fn default() -> Self {
        Self {
            id: 0,
            alpha_channel: None,
            animated: None,
            checksum: None,
            height: None,
            image_id: None,
            url: None,
            width: None,
        }
    }
}

impl FromId for PlatformLogo {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PlatformLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PlatformLogo [{}]", self.id)?;
        if let Some(ref img_id) = self.image_id {
            writeln!(f, "  Image ID: {}", img_id)?;
        }
        if let (Some(w), Some(h)) = (self.width, self.height) {
            writeln!(f, "  Dimensions: {}×{}", w, h)?;
        }
        if let Some(animated) = self.animated {
            writeln!(f, "  Animated: {}", animated)?;
        }
        if let Some(alpha) = self.alpha_channel {
            writeln!(f, "  Alpha Channel: {}", alpha)?;
        }
        Ok(())
    }
}
