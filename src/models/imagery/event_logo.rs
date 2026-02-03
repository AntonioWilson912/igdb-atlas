//! # Event Logo Model
//!
//! Represents a event logo from the IGDB v4 `/event_logos` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::imagery::EventLogo;
//!
//! let json = r#"{
//!     "id": 1,
//!     "image_id": "event_abc123",
//!     "width": 200,
//!     "height": 100
//! }"#;
//!
//! let logo: EventLogo = serde_json::from_str(json).unwrap();
//! assert_eq!(logo.image_id, Some("event_abc123".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    common::EventRef,
    id_or_object::{FromId, deserialize_id_or_object},
};

/// An event logo image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLogo {
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

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The event associated with this logo.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub event: Option<EventRef>,

    /// Image height in pixels.
    #[serde(default)]
    pub height: Option<u32>,

    /// The image ID used to construct IGDB image URLs.
    #[serde(default)]
    pub image_id: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// Direct URL to the image as stored by IGDB.
    #[serde(default)]
    pub url: Option<String>,

    /// Image width in pixels.
    #[serde(default)]
    pub width: Option<u32>,
}

impl Default for EventLogo {
    fn default() -> Self {
        Self {
            id: 0,
            alpha_channel: None,
            animated: None,
            checksum: None,
            created_at: None,
            event: None,
            height: None,
            image_id: None,
            updated_at: None,
            url: None,
            width: None,
        }
    }
}

impl FromId for EventLogo {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for EventLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EventLogo [{}]", self.id)?;
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
