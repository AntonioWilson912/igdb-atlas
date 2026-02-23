//! # Game Models
//!
//! Core game entity and game-specific sub-entities.
//!
//! | Module | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`game`] | `/games` | The primary [`Game`] model |
//! | [`alternative_name`] | `/alternative_names` | International / alternative titles |
//! | [`content_safety_rating`] | `/content_safety_ratings` | The safety rating of content |
//! | [`content_safety_rating_dimension`] | `/content_safety_rating_dimensions` | The safety rating dimension of content |
//! | [`external_game`] | `/external_games` | Listings on external storefronts |
//! | [`external_game_source`] | `/external_game_sources` | Storefront reference table |
//! | [`game_content_safety_rating`] | `/game_content_safety_ratings` | The safety rating of game content |
//! | [`game_engine`] | `/game_engines` | Video game engines |
//! | [`game_localization`] | `/game_localizations` | Localized game info |
//! | [`game_mode`] | `/game_modes` | Play modes (single / multiplayer) |
//! | [`game_release_format`] | `/game_release_formats` | Release formats (Digital / Physical) |
//! | [`game_status`] | `/game_statuses` | Release-status reference table |
//! | [`game_time_to_beat`] | `/game_time_to_beats` | Completion-time averages |
//! | [`game_type`] | `/game_types` | Game type (Main / DLC / Expansion …) |
//! | [`game_version`] | `/game_versions` | Edition / version groupings |
//! | [`game_version_feature`] | `/game_version_features` | Per-version feature definitions |
//! | [`game_version_feature_value`] | `/game_version_feature_values` | Feature flag values |
//! | [`game_video`] | `/game_videos` | Associated YouTube videos |
//! | [`genre`] | `/genres` | Game genres |
//! | [`involved_company`] | `/involved_companies` | Company ↔ game role mapping |
//! | [`keyword`] | `/keywords` | Tag keywords |
//! | [`multiplayer_mode`] | `/multiplayer_modes` | Multiplayer-support details |
//! | [`player_perspective`] | `/player_perspectives` | Camera / view perspectives |
//! | [`popularity_primitive`] | `/popularity_primitives` | Available primitives with source and popularity type |
//! | [`popularity_type`] | `/popularity_types` | What type of popularity indicator the value is |
//! | [`region`] | `/regions` | A region for game localization |
//! | [`release_date`] | `/release_dates` | Per-platform release dates |
//! | [`release_date_region`] | `/release_date_regions` | Region reference table |
//! | [`release_date_status`] | `/release_date_statuses` | Release-date status reference |
//!
//! Image models (covers, screenshots, engine logos) live in
//! [`crate::models::imagery`].  Language models live in
//! [`crate::models::languages`].

pub mod alternative_name;
pub mod content_safety_rating;
pub mod content_safety_rating_dimension;
pub mod external_game;
pub mod external_game_source;
pub mod game;
pub mod game_content_safety_rating;
pub mod game_engine;
pub mod game_localization;
pub mod game_mode;
pub mod game_release_format;
pub mod game_status;
pub mod game_time_to_beat;
pub mod game_type;
pub mod game_version;
pub mod game_version_feature;
pub mod game_version_feature_value;
pub mod game_video;
pub mod genre;
pub mod involved_company;
pub mod keyword;
pub mod multiplayer_mode;
pub mod player_perspective;
pub mod popularity_primitive;
pub mod popularity_type;
pub mod region;
pub mod release_date;
pub mod release_date_region;
pub mod release_date_status;

pub use alternative_name::AlternativeName;
pub use content_safety_rating::ContentSafetyRating;
pub use content_safety_rating_dimension::ContentSafetyRatingDimension;
pub use external_game::ExternalGame;
pub use external_game_source::ExternalGameSource;
pub use game::Game;
pub use game_content_safety_rating::GameContentSafetyRating;
pub use game_engine::GameEngine;
pub use game_localization::GameLocalization;
pub use game_mode::GameMode;
pub use game_release_format::GameReleaseFormat;
pub use game_status::GameStatus;
pub use game_time_to_beat::GameTimeToBeat;
pub use game_type::GameType;
pub use game_version::GameVersion;
pub use game_version_feature::GameVersionFeature;
pub use game_version_feature_value::GameVersionFeatureValue;
pub use game_video::GameVideo;
pub use genre::Genre;
pub use involved_company::InvolvedCompany;
pub use keyword::Keyword;
pub use multiplayer_mode::MultiplayerMode;
pub use player_perspective::PlayerPerspective;
pub use popularity_primitive::PopularityPrimitive;
pub use popularity_type::PopularityType;
pub use region::Region;
pub use release_date::ReleaseDate;
pub use release_date_region::ReleaseDateRegion;
pub use release_date_status::ReleaseDateStatus;
