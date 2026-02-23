//! # Screenshot Model
//!
//! Represents a game screenshot from the IGDB v4 `/screenshots` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::imagery::{Screenshot, UrlFor};
//!
//! let json = r#"{
//!     "id": 42,
//!     "image_id": "sc_abc123",
//!     "width": 1920,
//!     "height": 1080,
//!     "game": 1942
//! }"#;
//!
//! let ss: Screenshot = serde_json::from_str(json).unwrap();
//! assert_eq!(
//!     ss.url("screenshot_big"),
//!     Some("//images.igdb.com/igdb/image/upload/t_screenshot_big/sc_abc123.jpg".to_string()),
//! );
//! ```

use crate::models::id_or_object::FromId;
use serde::{Deserialize, Serialize};

/// A game screenshot image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screenshot {
    /// Unique screenshot identifier.
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

    /// The game this screenshot belongs to.
    #[serde(default)]
    pub game: Option<u64>,

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

impl Screenshot {
    /// Returns `true` if the screenshot is HD (width ≥ 1280).
    pub fn is_hd(&self) -> bool {
        self.width.map(|w| w >= 1280).unwrap_or(false)
    }
}

impl Default for Screenshot {
    fn default() -> Self {
        Self {
            id: 0,
            alpha_channel: None,
            animated: None,
            checksum: None,
            game: None,
            height: None,
            image_id: None,
            url: None,
            width: None,
        }
    }
}

impl FromId for Screenshot {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Screenshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Screenshot [{}]", self.id)?;
        if let Some(ref img_id) = self.image_id {
            writeln!(f, "  Image ID: {}", img_id)?;
        }
        if let (Some(w), Some(h)) = (self.width, self.height) {
            writeln!(f, "  Dimensions: {}×{}", w, h)?;
        }
        if let Some(game) = self.game {
            writeln!(f, "  Game ID: {}", game)?;
        }
        if let Some(animated) = self.animated {
            writeln!(f, "  Animated: {}", animated)?;
        }
        Ok(())
    }
}
