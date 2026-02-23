//! # Character Endpoints
//!
//! Fluent interfaces for IGDB character-related endpoints.
//!
//! ## Endpoints
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`CharactersEndpoint`] | `/characters` | [`Character`] |
//! | [`CharacterGendersEndpoint`] | `/character_genders` | [`CharacterGender`] |
//! | [`CharacterSpeciesEndpoint`] | `/character_species` | [`CharacterSpecies`] |

use crate::models::characters::{Character, CharacterGenderRecord, CharacterSpeciesRecord};

define_endpoint! {
    /// `/characters` - searchable.
    pub struct CharactersEndpoint => "characters", Character, searchable
}

define_endpoint! {
    /// `/character_genders` - not searchable.
    pub struct CharacterGendersEndpoint => "character_genders", CharacterGenderRecord
}

define_endpoint! {
    /// `/character_species` - not searchable.
    pub struct CharacterSpeciesEndpoint => "character_species", CharacterSpeciesRecord
}
