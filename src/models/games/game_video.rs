//! # Game Video Model
//!
//! Represents a video associated with a game from the IGDB v4
//! `/game_videos` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameVideo;
//!
//! let json = r#"{
//!     "id": 1,
//!     "game": 1942,
//!     "name": "Launch Trailer",
//!     "video_id": "dQw4w9WgXcQ"
//! }"#;
//!
//! let video: GameVideo = serde_json::from_str(json).unwrap();
//! assert_eq!(video.name, Some("Launch Trailer".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::id_or_object::{FromId, deserialize_id_or_object},
};

/// A video associated with a game. The `video_id` field contains a
/// YouTube video identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVideo {
    /// Unique video identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Reference ID to the game this video is associated with.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// The name of the video.
    #[serde(default)]
    pub name: Option<String>,

    /// The external ID of the video (YouTube video ID).
    #[serde(default)]
    pub video_id: Option<String>,
}

impl GameVideo {
    /// Returns the video name or `"Unknown Video"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Video")
    }

    /// Returns the full YouTube URL, or `None` if `video_id` is absent.
    pub fn youtube_url(&self) -> Option<String> {
        self.video_id
            .as_ref()
            .map(|vid| format!("https://www.youtube.com/watch?v={}", vid))
    }

    /// Returns the YouTube thumbnail URL, or `None` if `video_id` is absent.
    pub fn youtube_thumbnail(&self) -> Option<String> {
        self.video_id
            .as_ref()
            .map(|vid| format!("https://img.youtube.com/vi/{}/hqdefault.jpg", vid))
    }
}

impl Default for GameVideo {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            game: None,
            name: None,
            video_id: None,
        }
    }
}

impl FromId for GameVideo {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameVideo [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref game) = self.game {
            writeln!(f, "  Game ID: {}", game)?;
        }
        if let Some(ref video_id) = self.video_id {
            writeln!(f, "  Video ID: {}", video_id)?;
            writeln!(f, "  YouTube: https://www.youtube.com/watch?v={}", video_id)?;
        }
        Ok(())
    }
}
