//! # Games Endpoint
//!
//! Fluent interfaces for IGDB game-related endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`AlternativeNamesEndpoint`] | `/alternative_names` | [`AlternativeName`] |
//! | [`ExternalGamesEndpoint`] | `/external_games` | [`ExternalGame`] |
//! | [`ExternalGameSourcesEndpoint`] | `/external_game_sources` | [`ExternalGameSource`] |
//! | [`GamesEndpoint`] | `/games` | [`Game`] |
//! | [`GameEnginesEndpoint`] | `/game_engines` | [`GameEngine`] |
//! | [`GameLocalizationsEndpoint`] | `/game_localizations` | [`GameLocalization`] |
//! | [`GameModesEndpoint`] | `/game_modes` | [`GameMode`] |
//! | [`GameReleaseFormatsEndpoint`] | `/game_release_formats` | [`GameReleaseFormat`] |
//! | [`GameStatusesEndpoint`] | `/game_statuses` | [`GameStatus`] |
//! | [`GameTimeToBeatsEndpoint`] | `/game_time_to_beat` | [`GameTimeToBeat`] |
//! | [`GameTypesEndpoint`] | `/game_types` | [`GameType`] |
//! | [`GameVersionsEndpoint`] | `/game_versions` | [`GameVersion`] |
//! | [`GameVersionFeaturesEndpoint`] | `/game_version_features` | [`GameVersionFeature`] |
//! | [`GameVersionFeatureValuesEndpoint`] | `/game_version_feature_values` | [`GameVersionFeatureValue`] |
//! | [`GameVideosEndpoint`] | `/game_videos` | [`GameVideo`] |
//! | [`GenresEndpoint`] | `/genres` | [`Genre`] |
//! | [`InvolvedCompaniesEndpoint`] | `/involved_companies` | [`InvolvedCompany`] |
//! | [`KeywordsEndpoint`] | `/keywords` | [`Keyword`] |
//! | [`MultiplayerModesEndpoint`] | `/multiplayer_modes` | [`MultiplayerMode`] |
//! | [`PlayerPerspectivesEndpoint`] | `/player_perspectives` | [`PlayerPerspective`] |
//! | [`PopularityPrimitivesEndpoint`] | `/popularity_primitives` | [`PopularityPrimitive`] |
//! | [`PopularityTypesEndpoint`] | `/popularity_types` | [`PopularityType`] |
//! | [`RegionsEndpoint`] | `/regions` | [`Region`] |
//! | [`ReleaseDatesEndpoint`] | `/release_dates` | [`ReleaseDate`] |
//! | [`ReleaseDateRegionsEndpoint`] | `/release_date_regions` | [`ReleaseDateRegion`] |
//! | [`ReleaseDateStatusesEndpoint`] | `/release_date_statuses` | [`ReleaseDateStatus`] |

use crate::models::games::{
    AlternativeName, ExternalGame, ExternalGameSource, Game, GameEngine, GameLocalization,
    GameMode, GameReleaseFormat, GameStatus, GameTimeToBeat, GameType, GameVersion,
    GameVersionFeature, GameVersionFeatureValue, GameVideo, Genre, InvolvedCompany, Keyword,
    MultiplayerMode, PlayerPerspective, PopularityPrimitive, PopularityType, Region, ReleaseDate,
    ReleaseDateRegion, ReleaseDateStatus,
};

define_endpoint! {
    /// `alternative_names - not searchable.
    pub struct AlternativeNamesEndpoint => "alternative_names", AlternativeName, name_filterable
}

define_endpoint! {
    /// `external_games - not searchable.
    pub struct ExternalGamesEndpoint => "external_games", ExternalGame
}

define_endpoint! {
    /// `external_game_sources - not searchable.
    pub struct ExternalGameSourcesEndpoint => "external_game_sources", ExternalGameSource, name_filterable
}

define_endpoint! {
    /// `games - searchable
    pub struct GamesEndpoint => "games", Game, searchable
}

define_endpoint! {
    /// `game_engines - not searchable.
    pub struct GameEnginesEndpoint => "game_engines", GameEngine
}

define_endpoint! {
    /// `game_localizations - not searchable.
    pub struct GameLocalizationsEndpoint => "game_localizations", GameLocalization, name_filterable
}

define_endpoint! {
    /// `game_modes - not searchable.
    pub struct GameModesEndpoint => "game_modes", GameMode, name_filterable
}

define_endpoint! {
    /// `game_release_formats - not searchable.
    pub struct GameReleaseFormatsEndpoint => "game_release_formats", GameReleaseFormat
}

define_endpoint! {
    /// `game_statuses - not searchable.
    pub struct GameStatusesEndpoint => "game_statuses", GameStatus
}

define_endpoint! {
    /// `game_time_to_beat - not searchable.
    pub struct GameTimeToBeatsEndpoint => "game_time_to_beat", GameTimeToBeat
}

define_endpoint! {
    /// `game_types - not searchable.
    pub struct GameTypesEndpoint => "game_types", GameType
}

define_endpoint! {
    /// `game_versions - not searchable.
    pub struct GameVersionsEndpoint => "game_versions", GameVersion
}

define_endpoint! {
    /// `game_version_features - not searchable.
    pub struct GameVersionFeaturesEndpoint => "game_version_features", GameVersionFeature
}

define_endpoint! {
    /// `game_version_feature_values - not searchable.
    pub struct GameVersionFeatureValuesEndpoint => "game_version_feature_values", GameVersionFeatureValue
}

define_endpoint! {
    /// `game_videos - not searchable.
    pub struct GameVideosEndpoint => "game_videos", GameVideo, name_filterable
}

define_endpoint! {
    /// `genres - not searchable.
    pub struct GenresEndpoint => "genres", Genre, name_filterable
}

define_endpoint! {
    /// `involved_companies - not searchable.
    pub struct InvolvedCompaniesEndpoint => "involved_companies", InvolvedCompany
}

define_endpoint! {
    /// `keywords - not searchable.
    pub struct KeywordsEndpoint => "keywords", Keyword, name_filterable
}

define_endpoint! {
    /// `multiplayer_modes - not searchable.
    pub struct MultiplayerModesEndpoint => "multiplayer_modes", MultiplayerMode
}

define_endpoint! {
    /// `player_perspectives - not searchable.
    pub struct PlayerPerspectivesEndpoint => "player_perspectives", PlayerPerspective, name_filterable
}

define_endpoint! {
    /// `popularity_primitives - not searchable.
    pub struct PopularityPrimitivesEndpoint => "popularity_primitives", PopularityPrimitive
}

define_endpoint! {
    /// `popularity_types - not searchable.
    pub struct PopularityTypesEndpoint => "popularity_types", PopularityType, name_filterable
}

define_endpoint! {
    /// `regions - not searchable.
    pub struct RegionsEndpoint => "regions", Region, name_filterable
}

define_endpoint! {
    /// `release_dates - not searchable.
    pub struct ReleaseDatesEndpoint => "release_dates", ReleaseDate
}

define_endpoint! {
    /// `release_date_regions - not searchable.
    pub struct ReleaseDateRegionsEndpoint => "release_date_regions", ReleaseDateRegion
}

define_endpoint! {
    /// `release_date_statuses - not searchable.
    pub struct ReleaseDateStatusesEndpoint => "release_date_statuses", ReleaseDateStatus, name_filterable
}
