//! # Artwork Model
//!
//! Represents an artwork image from the IGDB v4
//! `/artworks` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//!
//! use igdb_atlas::models::imagery::{Artwork, UrlFor};
//!
//! let json = r#"{
//!     "id": 42,
//!     "image_id": "char_abc123",
//!     "width": 200,
//!     "height": 300,
//!     "animated": false,
//!     "alpha_channel": true
//! }"#;
//!
//! let artwork: Artwork = serde_json::from_str(json).unwrap();
//! assert_eq!(artwork.image_id, Some("char_abc123".to_string()));
//! assert_eq!(
//!     artwork.url("thumb"),
//!     Some("//images.igdb.com/igdb/image/upload/t_thumb/char_abc123.jpg".to_string()),
//! );
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        id_or_object::{FromId, deserialize_id_or_object},
        imagery::ArtworkType,
    },
};

/// An artwork image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artwork {
    /// Unique mug shot identifier.
    pub id: u64,

    /// Whether the image has an alpha channel (transparency).
    #[serde(default)]
    pub alpha_channel: Option<bool>,

    /// Whether the image is animated (e.g. GIF).
    #[serde(default)]
    pub animated: Option<bool>,

    /// The artwork type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub artwork_type: Option<ArtworkType>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The game this artwork is associated with.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

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

impl Artwork {
    /// Returns the image ID or `"Unknown Artwork"`.
    pub fn display_name(&self) -> &str {
        self.image_id.as_deref().unwrap_or("Unknown Artwork")
    }
}

impl Default for Artwork {
    fn default() -> Self {
        Self {
            id: 0,
            alpha_channel: None,
            animated: None,
            artwork_type: None,
            checksum: None,
            game: None,
            height: None,
            image_id: None,
            url: None,
            width: None,
        }
    }
}

impl FromId for Artwork {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Artwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Artwork [{}]", self.id)?;
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
