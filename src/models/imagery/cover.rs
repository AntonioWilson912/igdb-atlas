//! # Cover Model
//!
//! Represents a game cover image from the IGDB v4 `/covers` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::imagery::{Cover, UrlFor};
//!
//! let json = r#"{
//!     "id": 100,
//!     "image_id": "co1abc",
//!     "width": 264,
//!     "height": 374,
//!     "game": 1942
//! }"#;
//!
//! let cover: Cover = serde_json::from_str(json).unwrap();
//! assert_eq!(
//!     cover.url("cover_big"),
//!     Some("//images.igdb.com/igdb/image/upload/t_cover_big/co1abc.jpg".to_string()),
//! );
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    common::{GameLocalizationRef, GameRef},
    id_or_object::{FromId, deserialize_id_or_object},
};

/// Cover art for a game or game localization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cover {
    /// Unique cover identifier.
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

    /// The game this cover belongs to.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<GameRef>,

    /// The game localization this cover belongs to.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game_localization: Option<GameLocalizationRef>,

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

impl Cover {
    /// Returns `true` if this cover belongs to a game localization
    /// rather than a game directly.
    pub fn is_localized(&self) -> bool {
        self.game_localization.is_some()
    }
}

impl Default for Cover {
    fn default() -> Self {
        Self {
            id: 0,
            alpha_channel: None,
            animated: None,
            checksum: None,
            game: None,
            game_localization: None,
            height: None,
            image_id: None,
            url: None,
            width: None,
        }
    }
}

impl FromId for Cover {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Cover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cover [{}]", self.id)?;
        if let Some(ref img_id) = self.image_id {
            writeln!(f, "  Image ID: {}", img_id)?;
        }
        if let (Some(w), Some(h)) = (self.width, self.height) {
            writeln!(f, "  Dimensions: {}×{}", w, h)?;
        }
        if let Some(ref game) = self.game {
            writeln!(f, "  Game ID: {}", game.id)?;
        }
        if let Some(ref loc) = self.game_localization {
            writeln!(f, "  Localization ID: {}", loc.id)?;
        }
        Ok(())
    }
}
