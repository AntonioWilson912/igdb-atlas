//! # Popularity Primitive Model
//!
//! Represents a popularity primitive from the IGDB v4
//! `/popularity_primitives` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::PopularityPrimitive;
//!
//! let json = r#"{
//!     "id": 930694,
//!     "game_id": 305006,
//!     "value": 0.000248297441802,
//!     "popularity_type": 9,
//!     "popularity_source": 1
//! }"#;
//! let pp: PopularityPrimitive = serde_json::from_str(json).unwrap();
//! assert_eq!(pp.popularity_type.as_ref().map(|o| o.id), Some(9));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::{ExternalGameSource, PopularityType},
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// Available primitives with their source and popularity type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularityPrimitive {
    /// Unique primitive identifier.
    pub id: u64,

    /// When the primitive was calculated.
    #[serde(default)]
    pub calculated_at: Option<i64>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The external game source.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub external_popularity_source: Option<ExternalGameSource>,

    /// The game associated with the primitive.
    #[serde(default)]
    pub game_id: Option<i64>,

    /// The popularity type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub popularity_type: Option<PopularityType>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The value of the primitive.
    #[serde(default)]
    pub value: Option<f64>,
}

impl PopularityPrimitive {
    /// Returns the popularity value or 0.0.
    pub fn value_or_default(&self) -> f64 {
        self.value.unwrap_or(0.0)
    }
}

impl Default for PopularityPrimitive {
    fn default() -> Self {
        Self {
            id: 0,
            calculated_at: None,
            checksum: None,
            created_at: None,
            external_popularity_source: None,
            game_id: None,
            popularity_type: None,
            updated_at: None,
            value: None,
        }
    }
}

impl FromId for PopularityPrimitive {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PopularityPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PopularityPrimitive [{}]", self.id)?;
        if let Some(game_id) = self.game_id {
            write!(f, " Game ID: {}", game_id)?;
        }
        if let Some(value) = self.value {
            write!(f, " Value: {}", value)?;
        }
        if let Some(ts) = self.calculated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " calculated {}", date)?;
            }
        }
        writeln!(f)
    }
}
