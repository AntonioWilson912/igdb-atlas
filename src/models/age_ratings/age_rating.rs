//! # Age Rating Model
//!
//! Represents an age rating from the IGDB v4 `/age_ratings` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::age_ratings::AgeRating;
//!
//! let json = r#"{
//!     "id": 1,
//!     "organization": 5,
//!     "rating_category": 10,
//!     "synopsis": "Contains mild violence"
//! }"#;
//!
//! let rating: AgeRating = serde_json::from_str(json).unwrap();
//! assert_eq!(rating.organization.as_ref().map(|o| o.id), Some(5));
//! assert_eq!(rating.rating_category.as_ref().map(|c| c.id), Some(10));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    age_ratings::{AgeRatingCategory, AgeRatingContentDescriptionV2, AgeRatingOrganization},
    id_or_object::{FromId, deserialize_id_or_object, deserialize_id_or_object_vec},
};

/// An age rating entry from a rating organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRating {
    /// Unique age rating identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The organization that has issued a specific rating.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub organization: Option<AgeRatingOrganization>,

    /// The category of a rating.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub rating_category: Option<AgeRatingCategory>,

    /// Content description IDs from the `/age_rating_content_descriptions_v2` endpoint.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub rating_content_descriptions: Option<Vec<AgeRatingContentDescriptionV2>>,

    /// URL to the rating badge / cover image.
    #[serde(default)]
    pub rating_cover_url: Option<String>,

    /// Free-text synopsis motivating the rating.
    #[serde(default)]
    pub synopsis: Option<String>,
}

impl Default for AgeRating {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            organization: None,
            rating_category: None,
            rating_content_descriptions: None,
            rating_cover_url: None,
            synopsis: None,
        }
    }
}

impl FromId for AgeRating {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for AgeRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgeRating [{}]", self.id)?;
        if let Some(ref org) = self.organization {
            write!(f, " org={}", org)?;
        }
        if let Some(ref rc) = self.rating_category {
            write!(f, " rating_cat={}", rc)?;
        }
        if let Some(ref url) = self.rating_cover_url {
            write!(f, " cover={}", url)?;
        }
        if let Some(ref syn) = self.synopsis {
            let short = if syn.len() > 80 {
                format!("{}...", &syn[..80])
            } else {
                syn.clone()
            };
            write!(f, " \"{}\"", short)?;
        }
        writeln!(f)
    }
}
