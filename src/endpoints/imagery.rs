//! # Imagery Endpoint
//!
//! Fluent interfaces for the IGDB image endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`ArtworksEndpoint`] | `/artworks` | [`Artwork`] |
//! | [`ArtworkTypesEndpoint`] | `/artwork_types` | [`ArtworkType`] |
//! | [`CharacterMugShotsEndpoint`] | `/character_mug_shots` | [`CharacterMugShot`] |
//! | [`CompanyLogosEndpoint`] | `/company_logos` | [`CompanyLogo`] |
//! | [`CoversEndpoint`] | `/covers` | [`Cover`] |
//! | [`EventLogosEndpoint`] | `/event_logos` | [`EventLogo`] |
//! | [`GameEngineLogosEndpoint`] | `/game_engine_logos` | [`GameEngineLogo`] |
//! | [`PlatformLogosEndpoint`] | `/platform_logos` | [`PlatformLogo`] |
//! | [`ScreenshotsEndpoint`] | `/screenshots` | [`Screenshot`] |

use crate::models::imagery::{
    Artwork, ArtworkType, CharacterMugShot, CompanyLogo, Cover, EventLogo, GameEngineLogo,
    PlatformLogo, Screenshot,
};

define_endpoint! {
    /// `/artworks` - not searchable.
    pub struct ArtworksEndpoint => "artworks", Artwork
}

define_endpoint! {
    /// `/artwork_types` - not searchable.
    pub struct ArtworkTypesEndpoint => "artwork_types", ArtworkType, name_filterable
}

define_endpoint! {
    /// `/character_mug_shots` - not searchable.
    pub struct CharacterMugShotsEndpoint => "character_mug_shots", CharacterMugShot
}

define_endpoint! {
    /// `/company_logos` - not searchable.
    pub struct CompanyLogosEndpoint => "company_logos", CompanyLogo
}

define_endpoint! {
    /// `/covers` - not searchable.
    pub struct CoversEndpoint => "covers", Cover
}

define_endpoint! {
    /// `/event_logos` - not searchable.
    pub struct EventLogosEndpoint => "event_logos", EventLogo
}

define_endpoint! {
    /// `/game_engine_logos` - not searchable.
    pub struct GameEngineLogosEndpoint => "game_engine_logos", GameEngineLogo
}

define_endpoint! {
    /// `/platform_logos` - not searchable.
    pub struct PlatformLogosEndpoint => "platform_logos", PlatformLogo
}

define_endpoint! {
    /// `/screenshots` - not searchable.
    pub struct ScreenshotsEndpoint => "screenshots", Screenshot
}
