//! # Models
//!
//! Strongly-typed response models for IGDB API data, organised by domain.
//!
//! | Domain | What it covers |
//! |--------|----------------|
//! | [`age_ratings`] | Rating organisations, categories, content descriptions |
//! | [`characters`] | Characters, gender & species reference tables |
//! | [`collections`] | Game series / collections |
//! | [`companies`] | Developers, publishers, studios |
//! | [`common`] | Lightweight `…Ref` types shared across domains |
//! | [`date_formats`] | Date formats for release dates |
//! | [`events`] | Industry events |
//! | [`franchises`] | Game franchises |
//! | [`games`] | Core game entity and game-specific sub-entities |
//! | [`imagery`] | All image models: covers, screenshots, logos, portraits |
//! | [`languages`] | Language definitions and per-game language-support records |
//! | [`networks`] | Social networks |
//! | [`platforms`] | Platforms and platform families |
//! | [`searches`] | General search support for characters, collections, companies, games, and platforms |
//! | [`themes`] | Narrative / atmospheric themes |
//! | [`websites`] | Website URLs and type reference table |
//!
//! ## Serialization & optional fields
//!
//! All models implement `serde::Deserialize` and `serde::Serialize`.
//! Every field except `id` is `Option<T>`: the IGDB API only returns fields
//! that were explicitly listed in the query's `fields` clause, so anything
//! not requested deserializes to `None`.
//!
//! ## ID-or-Object polymorphism
//!
//! Nested references arrive as either bare integer IDs or full objects
//! depending on whether the field was expanded.  The [`id_or_object`] module
//! provides the deserialization helpers that handle both forms transparently.

pub mod age_ratings;
pub mod characters;
pub mod collections;
pub mod common;
pub mod companies;
pub mod date_formats;
pub mod events;
pub mod franchises;
pub mod games;
pub mod id_or_object;
pub mod imagery;
pub mod languages;
pub mod networks;
pub mod platforms;
pub mod searches;
pub mod themes;
pub mod timestamp;
pub mod websites;
