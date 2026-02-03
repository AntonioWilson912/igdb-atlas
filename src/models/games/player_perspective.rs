//! # Player Perspective Model
//!
//! Represents a player perspective from the IGDB v4
//! `/player_perspectives` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::PlayerPerspective;
//!
//! let json = r#"{"id": 1, "name": "First person", "slug": "first-person"}"#;
//! let pp: PlayerPerspective = serde_json::from_str(json).unwrap();
//! assert_eq!(pp.display_name(), "First person");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A player perspective describing the camera view in a video game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPerspective {
    /// Unique perspective identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Perspective name.
    #[serde(default)]
    pub name: Option<String>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The IGDB page URL.
    #[serde(default)]
    pub url: Option<String>,
}

impl PlayerPerspective {
    /// Returns the perspective name or `"Unknown Perspective"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Perspective")
    }

    /// Returns `true` if this is first-person perspective (ID 1).
    pub fn is_first_person(&self) -> bool {
        self.id == 1
    }

    /// Returns `true` if this is third-person perspective (ID 2).
    pub fn is_third_person(&self) -> bool {
        self.id == 2
    }

    /// Returns `true` if this is a VR perspective (ID 7).
    pub fn is_vr(&self) -> bool {
        self.id == 7
    }
}

impl Default for PlayerPerspective {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            slug: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for PlayerPerspective {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for PlayerPerspective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerPerspective [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            write!(f, " ({})", slug)?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                write!(f, " updated {}", date)?;
            }
        }
        writeln!(f)
    }
}
