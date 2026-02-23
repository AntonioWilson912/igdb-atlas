//! # Game Engine Model
//!
//! Represents a video game engine from the IGDB v4 `/game_engines` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::GameEngine;
//!
//! let json = r#"{
//!     "id": 1,
//!     "name": "Unreal Engine",
//!     "description": "A game engine developed by Epic Games",
//!     "slug": "unreal-engine"
//! }"#;
//!
//! let engine: GameEngine = serde_json::from_str(json).unwrap();
//! assert_eq!(engine.name, Some("Unreal Engine".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Company, Platform,
    models::{
        id_or_object::{FromId, deserialize_id_or_object_vec},
        imagery::GameEngineLogo,
        timestamp::format_timestamp,
    },
};

/// A video game engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEngine {
    /// Unique game engine identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Company IDs that used this game engine.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub companies: Option<Vec<Company>>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Description of the game engine.
    #[serde(default)]
    pub description: Option<String>,

    /// Logo of the game engine.
    #[serde(default)]
    pub logo: Option<GameEngineLogo>,

    /// Name of the game engine.
    #[serde(default)]
    pub name: Option<String>,

    /// Platform IDs this game engine was deployed on.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub platforms: Option<Vec<Platform>>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The website address (URL) of the item.
    #[serde(default)]
    pub url: Option<String>,
}

impl GameEngine {
    /// Returns the engine name or `"Unknown Engine"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Engine")
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            companies: None,
            created_at: None,
            description: None,
            logo: None,
            name: None,
            platforms: None,
            slug: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for GameEngine {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for GameEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameEngine [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref desc) = self.description {
            let truncated = if desc.len() > 200 {
                format!("{}...", &desc[..200])
            } else {
                desc.clone()
            };
            writeln!(f, "  Description: {}", truncated)?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
        }
        if let Some(ref platforms) = self.platforms {
            if !platforms.is_empty() {
                writeln!(f, "  Platforms: {} (IDs)", platforms.len())?;
            }
        }
        if let Some(ref companies) = self.companies {
            if !companies.is_empty() {
                writeln!(f, "  Companies: {} (IDs)", companies.len())?;
            }
        }
        if let Some(ts) = self.created_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Added: {}", date)?;
            }
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Updated: {}", date)?;
            }
        }
        Ok(())
    }
}
