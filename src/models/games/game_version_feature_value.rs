//! # Game Version Feature Value Model
//!
//! Represents the bool/text value of a game version feature from the
//! IGDB v4 `/game_version_feature_values` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameVersionFeatureValue;
//! use igdb_atlas::models::games::game_version_feature_value::IncludedFeature;
//!
//! let json = r#"{
//!     "id": 1,
//!     "game": 1942,
//!     "game_feature": 5,
//!     "included_feature": 1
//! }"#;
//!
//! let value: GameVersionFeatureValue = serde_json::from_str(json).unwrap();
//! assert_eq!(value.included_feature, Some(IncludedFeature::Included));
//! assert!(value.is_included());
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        games::GameVersionFeature,
        id_or_object::{FromId, deserialize_id_or_object},
    },
};

/// Inclusion status for a game version feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u16", into = "u16")]
pub enum IncludedFeature {
    /// The feature is not present in this version.
    NotIncluded,
    /// The feature is fully included in this version.
    Included,
    /// The feature is available as a pre-order bonus only.
    PreorderOnly,
    /// An unrecognised value returned by the API.
    Unknown(u16),
}

impl IncludedFeature {
    /// Returns a human-readable description of the inclusion status.
    pub fn description(self) -> &'static str {
        match self {
            Self::NotIncluded => "Not Included",
            Self::Included => "Included",
            Self::PreorderOnly => "Pre-order Only",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl From<u16> for IncludedFeature {
    fn from(v: u16) -> Self {
        match v {
            0 => Self::NotIncluded,
            1 => Self::Included,
            2 => Self::PreorderOnly,
            other => Self::Unknown(other),
        }
    }
}

impl From<IncludedFeature> for u16 {
    fn from(f: IncludedFeature) -> Self {
        match f {
            IncludedFeature::NotIncluded => 0,
            IncludedFeature::Included => 1,
            IncludedFeature::PreorderOnly => 2,
            IncludedFeature::Unknown(v) => v,
        }
    }
}

impl std::fmt::Display for IncludedFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description())
    }
}

/// A feature value for a specific game version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVersionFeatureValue {
    /// Unique feature value identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Reference ID to the game this value refers to.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Reference ID to the game version feature.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game_feature: Option<GameVersionFeature>,

    /// Inclusion status (not included, included, or pre-order only).
    #[serde(default)]
    pub included_feature: Option<IncludedFeature>,

    /// The text value of this feature.
    #[serde(default)]
    pub note: Option<String>,
}

impl GameVersionFeatureValue {
    /// Returns `true` if the feature is fully included.
    pub fn is_included(&self) -> bool {
        self.included_feature == Some(IncludedFeature::Included)
    }

    /// Returns `true` if the feature is available as a pre-order bonus only.
    pub fn is_preorder_only(&self) -> bool {
        self.included_feature == Some(IncludedFeature::PreorderOnly)
    }
}

impl Default for GameVersionFeatureValue {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            game: None,
            game_feature: None,
            included_feature: None,
            note: None,
        }
    }
}

impl FromId for GameVersionFeatureValue {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameVersionFeatureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameVersionFeatureValue [{}]", self.id)?;
        if let Some(included) = self.included_feature {
            write!(f, " {}", included)?;
        }
        if let Some(ref note) = self.note {
            write!(f, " \"{}\"", note)?;
        }
        writeln!(f)
    }
}
