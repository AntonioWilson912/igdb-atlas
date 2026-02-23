//! # Game Content Safety Rating Model
//!
//! Represents a content safety rating for a game from the
//! IGDB v4 `/game_content_safety_ratings` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameContentSafetyRating;
//!
//! let json = r#"{
//! 	"id": 1,
//! 	"content_safety_rating": 3,
//! 	"content_safety_rating_dimension": 1,
//! 	"game": 1942
//! }"#;
//! let rating : GameContentSafetyRating = serde_json::from_str(json).unwrap();
//! assert_eq!(rating.content_safety_rating.as_ref().map(|o| o.id), Some(3));
//! assert_eq!(rating.content_safety_rating_dimension.as_ref().map(|o| o.id), Some(1));
//! assert_eq!(rating.game.as_ref().map(|o| o.id), Some(1942));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::{ContentSafetyRating, ContentSafetyRatingDimension, Game},
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A content safety rating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameContentSafetyRating {
    /// Unique source identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The content safety rating.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub content_safety_rating: Option<ContentSafetyRating>,

    /// The content safety rating dimension.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub content_safety_rating_dimension: Option<ContentSafetyRatingDimension>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The game associated with the content safety rating.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Default for GameContentSafetyRating {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            content_safety_rating: None,
            content_safety_rating_dimension: None,
            created_at: None,
            game: None,
            updated_at: None,
        }
    }
}

impl FromId for GameContentSafetyRating {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameContentSafetyRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameContentSafetyRating [{}]", self.id)?;
        if let Some(ref rating) = self.content_safety_rating {
            write!(f, " Rating: {}", rating)?;
        }
        if let Some(ref dimension) = self.content_safety_rating_dimension {
            write!(f, " Dimension: {}", dimension)?;
        }
        if let Some(ref game) = self.game {
            write!(f, " Game: {}", game)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", date)?;
            }
        }
        writeln!(f)
    }
}
