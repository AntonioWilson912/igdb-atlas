//! # Character Models
//!
//! Models for video game characters and related reference tables.
//!
//! | Module | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`character`] | `/characters` | The primary [`Character`] model |
//! | [`character_gender`] | `/character_genders` | Gender reference table |
//! | [`character_species`] | `/character_species` | Species reference table |
//!
//! Character portrait images live in [`crate::models::imagery::CharacterMugShot`].

pub mod character;
pub mod character_gender;
pub mod character_species;

pub use character::Character;
pub use character_gender::CharacterGenderRecord;
pub use character_species::CharacterSpeciesRecord;
