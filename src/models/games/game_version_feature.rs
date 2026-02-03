//! # Game Version Feature Model
//!
//! Represents a feature that differentiates game versions from the
//! IGDB v4 `/game_version_features` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameVersionFeature;
//!
//! let json = r#"{
//!     "id": 1,
//!     "title": "Includes Soundtrack",
//!     "category": 0,
//!     "position": 1
//! }"#;
//!
//! let feature: GameVersionFeature = serde_json::from_str(json).unwrap();
//! assert_eq!(feature.title, Some("Includes Soundtrack".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::GameVersionFeatureValue,
    id_or_object::{FromId, deserialize_id_or_object_vec},
};

/// Category type alias for game version features.
pub type GameVersionFeatureCategory = u16;

/// Helpers for interpreting feature category values.
pub struct GameVersionFeatureCategoryExt;

impl GameVersionFeatureCategoryExt {
    /// Returns a description of the category value.
    pub fn description(category: GameVersionFeatureCategory) -> &'static str {
        match category {
            0 => "Boolean",
            1 => "Description",
            _ => "Unknown",
        }
    }
}

/// A game version feature describing edition differences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVersionFeature {
    /// Unique feature identifier.
    pub id: u64,

    /// The category of the feature (0 = boolean, 1 = description).
    #[serde(default)]
    pub category: Option<u16>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The description of the feature.
    #[serde(default)]
    pub description: Option<String>,

    /// Position of this feature in the list of features.
    #[serde(default)]
    pub position: Option<u32>,

    /// The title of the feature.
    #[serde(default)]
    pub title: Option<String>,

    /// Feature value IDs containing the bool/text values.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub values: Option<Vec<GameVersionFeatureValue>>,
}

impl GameVersionFeature {
    /// Returns the feature title or `"Unknown Feature"`.
    pub fn display_name(&self) -> &str {
        self.title.as_deref().unwrap_or("Unknown Feature")
    }

    /// Returns `true` if this is a boolean-type feature (category 0).
    pub fn is_boolean(&self) -> bool {
        self.category == Some(0)
    }
}

impl Default for GameVersionFeature {
    fn default() -> Self {
        Self {
            id: 0,
            category: None,
            checksum: None,
            description: None,
            position: None,
            title: None,
            values: None,
        }
    }
}

impl FromId for GameVersionFeature {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameVersionFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameVersionFeature [{}]", self.id)?;
        if let Some(ref title) = self.title {
            writeln!(f, "  Title: {}", title)?;
        }
        if let Some(category) = self.category {
            writeln!(
                f,
                "  Category: {}",
                GameVersionFeatureCategoryExt::description(category)
            )?;
        }
        if let Some(ref desc) = self.description {
            writeln!(f, "  Description: {}", desc)?;
        }
        if let Some(position) = self.position {
            writeln!(f, "  Position: {}", position)?;
        }
        if let Some(ref values) = self.values {
            writeln!(f, "  Values: {} defined", values.len())?;
        }
        Ok(())
    }
}
