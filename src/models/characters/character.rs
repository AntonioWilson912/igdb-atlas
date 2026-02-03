//! # Character Model
//!
//! Represents a video game character from the IGDB v4 `/characters` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::characters::Character;
//!
//! let json = r#"{
//!     "id": 100,
//!     "name": "Mario",
//!     "description": "Nintendo's iconic plumber",
//!     "country_name": "Mushroom Kingdom",
//!     "games": [1, 2, 3]
//! }"#;
//!
//! let character: Character = serde_json::from_str(json).unwrap();
//! assert_eq!(character.name, Some("Mario".to_string()));
//! assert_eq!(character.game_count(), 3);
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        characters::CharacterGenderRecord,
        id_or_object::{FromId, deserialize_id_or_object, deserialize_id_or_object_vec},
        imagery::CharacterMugShot,
        timestamp::format_timestamp,
    },
};

/// A video game character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Unique character identifier.
    pub id: u64,

    /// Alternative names for the character.
    #[serde(default)]
    pub akas: Option<Vec<String>>,

    /// Reference ID to the `/character_genders` endpoint.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub character_gender: Option<CharacterGenderRecord>,

    /// Reference ID to the `/character_species` endpoint.
    #[serde(default)]
    pub character_species: Option<u64>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Character's country of origin.
    #[serde(default)]
    pub country_name: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Description of the character.
    #[serde(default)]
    pub description: Option<String>,

    /// Game IDs this character appears in.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub games: Option<Vec<Game>>,

    /// Reference ID to the character's portrait image
    /// (see [`crate::models::imagery::CharacterMugShot`]).
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub mug_shot: Option<CharacterMugShot>,

    /// Character's name.
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

impl Character {
    /// Returns the character name or `"Unknown Character"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Character")
    }

    /// Returns the number of games this character appears in.
    pub fn game_count(&self) -> usize {
        self.games.as_ref().map(|g| g.len()).unwrap_or(0)
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            id: 0,
            akas: None,
            character_gender: None,
            character_species: None,
            checksum: None,
            country_name: None,
            created_at: None,
            description: None,
            games: None,
            mug_shot: None,
            name: None,
            slug: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for Character {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Character [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref akas) = self.akas {
            if !akas.is_empty() {
                writeln!(f, "  Also Known As: {}", akas.join(", "))?;
            }
        }
        if let Some(ref desc) = self.description {
            let truncated = if desc.len() > 200 {
                format!("{}...", &desc[..200])
            } else {
                desc.clone()
            };
            writeln!(f, "  Description: {}", truncated)?;
        }
        if let Some(ref country) = self.country_name {
            writeln!(f, "  Country: {}", country)?;
        }
        if let Some(ref games) = self.games {
            writeln!(f, "  Appears in {} games", games.len())?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
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
