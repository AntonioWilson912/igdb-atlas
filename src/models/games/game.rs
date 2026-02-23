//! # Game Model
//!
//! Represents a game entry from the IGDB v4 `/games` endpoint.
//!
//! Nested references use [`deserialize_id_or_object_vec`] so the same
//! struct handles both unexpanded (bare IDs) and expanded (full objects)
//! forms.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::Game;
//!
//! let json = r#"{
//!     "id": 1942,
//!     "name": "The Witcher 3: Wild Hunt",
//!     "rating": 93.5,
//!     "summary": "An epic RPG adventure.",
//!     "game_modes": [{"id": 1, "name": "Single player"}],
//!     "genres": [{"id": 12, "name": "Role-playing Games"}]
//! }"#;
//!
//! let game: Game = serde_json::from_str(json).unwrap();
//! assert_eq!(game.id, 1942);
//! assert_eq!(game.name, Some("The Witcher 3: Wild Hunt".to_string()));
//! assert_eq!(game.rating, Some(93.5));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Collection, Franchise,
    models::{
        age_ratings::AgeRating,
        common::{GameRef, PlatformRef, ThemeRef},
        games::{
            AlternativeName, ExternalGame, GameEngine, GameLocalization, GameMode, GameStatus,
            GameType, GameVideo, Genre, InvolvedCompany, Keyword, MultiplayerMode,
            PlayerPerspective, ReleaseDate,
        },
        id_or_object::{FromId, deserialize_id_or_object, deserialize_id_or_object_vec},
        imagery::{Artwork, Cover, Screenshot, UrlFor},
        languages::LanguageSupport,
        timestamp::{format_timestamp, format_timestamp_pretty},
        websites::Website,
    },
};

/// The main Game model representing an IGDB v4 game entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique game identifier.
    pub id: u64,

    /// Age rating IDs or expanded age rating objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub age_ratings: Option<Vec<AgeRating>>,

    /// Average critic rating (0–100).
    #[serde(default)]
    pub aggregated_rating: Option<f64>,

    /// Number of external critic scores aggregated.
    #[serde(default)]
    pub aggregated_rating_count: Option<u64>,

    /// Alternative name IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub alternative_names: Option<Vec<AlternativeName>>,

    /// Artwork image IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub artworks: Option<Vec<Artwork>>,

    /// Bundle IDs that include this game.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub bundles: Option<Vec<GameRef>>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Collection IDs this game belongs to.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub collections: Option<Vec<Collection>>,

    /// Cover art ID or expanded cover object.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub cover: Option<Cover>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// DLC game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub dlcs: Option<Vec<GameRef>>,

    /// Expanded game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub expanded_games: Option<Vec<GameRef>>,

    /// Expansion game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub expansions: Option<Vec<GameRef>>,

    /// External game listing IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub external_games: Option<Vec<ExternalGame>>,

    /// Unix timestamp of the earliest known release date.
    #[serde(default)]
    pub first_release_date: Option<i64>,

    /// Fork game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub forks: Option<Vec<GameRef>>,

    /// Primary franchise ID.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub franchise: Option<Franchise>,

    /// All franchise IDs this game belongs to.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub franchises: Option<Vec<Franchise>>,

    /// Game engine IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub game_engines: Option<Vec<GameEngine>>,

    /// Game localization IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub game_localizations: Option<Vec<GameLocalization>>,

    /// Game mode IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub game_modes: Option<Vec<GameMode>>,

    /// The status of the game's release.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game_status: Option<GameStatus>,

    /// The type of game.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game_type: Option<GameType>,

    /// Genre IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub genres: Option<Vec<Genre>>,

    /// Number of follows / hypes before release.
    #[serde(default)]
    pub hypes: Option<u64>,

    /// Involved company IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub involved_companies: Option<Vec<InvolvedCompany>>,

    /// Keyword IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub keywords: Option<Vec<Keyword>>,

    /// Language support IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub language_supports: Option<Vec<LanguageSupport>>,

    /// Multiplayer mode IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub multiplayer_modes: Option<Vec<MultiplayerMode>>,

    /// The game's primary name.
    #[serde(default)]
    pub name: Option<String>,

    /// Parent game ID (for DLCs / editions).
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub parent_game: Option<GameRef>,

    /// Platform IDs or expanded platform reference objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub platforms: Option<Vec<PlatformRef>>,

    /// Player perspective IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub player_perspectives: Option<Vec<PlayerPerspective>>,

    /// Port game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub ports: Option<Vec<GameRef>>,

    /// Average user rating (0–100).
    #[serde(default)]
    pub rating: Option<f64>,

    /// Number of user rating submissions.
    #[serde(default)]
    pub rating_count: Option<u64>,

    /// Release date IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub release_dates: Option<Vec<ReleaseDate>>,

    /// Remake game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub remakes: Option<Vec<GameRef>>,

    /// Remaster game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub remasters: Option<Vec<GameRef>>,

    /// Screenshot IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub screenshots: Option<Vec<Screenshot>>,

    /// Similar game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub similar_games: Option<Vec<GameRef>>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Standalone expansion game IDs.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub standalone_expansions: Option<Vec<GameRef>>,

    /// Long-form story description.
    #[serde(default)]
    pub storyline: Option<String>,

    /// Short description of the game.
    #[serde(default)]
    pub summary: Option<String>,

    /// Tag numbers based on IGDB's tag system.
    #[serde(default)]
    pub tags: Option<Vec<i64>>,

    /// Theme IDs or expanded theme reference objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub themes: Option<Vec<ThemeRef>>,

    /// Weighted average of user and critic ratings.
    #[serde(default)]
    pub total_rating: Option<f64>,

    /// Combined count of user and critic rating submissions.
    #[serde(default)]
    pub total_rating_count: Option<u64>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The IGDB page URL.
    #[serde(default)]
    pub url: Option<String>,

    /// Parent version game ID (for edition variants).
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub version_parent: Option<GameRef>,

    /// Edition title (e.g. "Game of the Year Edition").
    #[serde(default)]
    pub version_title: Option<String>,

    /// Video IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub videos: Option<Vec<GameVideo>>,

    /// Website IDs or expanded objects.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub websites: Option<Vec<Website>>,
}

impl Game {
    /// Returns the game name or `"Unknown Game"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Game")
    }

    /// Returns a constructed cover URL at the given size, if a cover
    /// with an `image_id` is present.
    pub fn cover_url(&self, size: &str) -> Option<String> {
        self.cover.as_ref().and_then(|cover| cover.url(size))
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            id: 0,
            age_ratings: None,
            aggregated_rating: None,
            aggregated_rating_count: None,
            alternative_names: None,
            artworks: None,
            bundles: None,
            checksum: None,
            collections: None,
            cover: None,
            created_at: None,
            dlcs: None,
            expanded_games: None,
            expansions: None,
            external_games: None,
            first_release_date: None,
            forks: None,
            franchise: None,
            franchises: None,
            game_engines: None,
            game_localizations: None,
            game_modes: None,
            game_status: None,
            game_type: None,
            genres: None,
            hypes: None,
            involved_companies: None,
            keywords: None,
            language_supports: None,
            multiplayer_modes: None,
            name: None,
            parent_game: None,
            platforms: None,
            player_perspectives: None,
            ports: None,
            rating: None,
            rating_count: None,
            release_dates: None,
            remakes: None,
            remasters: None,
            screenshots: None,
            similar_games: None,
            slug: None,
            standalone_expansions: None,
            storyline: None,
            summary: None,
            tags: None,
            themes: None,
            total_rating: None,
            total_rating_count: None,
            updated_at: None,
            url: None,
            version_parent: None,
            version_title: None,
            videos: None,
            websites: None,
        }
    }
}

impl FromId for Game {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game [{}]", self.id)?;

        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref summary) = self.summary {
            let truncated = if summary.len() > 200 {
                format!("{}...", &summary[..200])
            } else {
                summary.clone()
            };
            writeln!(f, "  Summary: {}", truncated)?;
        }
        if let Some(ref storyline) = self.storyline {
            let truncated = if storyline.len() > 200 {
                format!("{}...", &storyline[..200])
            } else {
                storyline.clone()
            };
            writeln!(f, "  Storyline: {}", truncated)?;
        }
        if let Some(rating) = self.rating {
            write!(f, "  Rating: {:.1}", rating)?;
            if let Some(count) = self.rating_count {
                write!(f, " ({} votes)", count)?;
            }
            writeln!(f)?;
        }
        if let Some(agg) = self.aggregated_rating {
            write!(f, "  Critic Rating: {:.1}", agg)?;
            if let Some(count) = self.aggregated_rating_count {
                write!(f, " ({} reviews)", count)?;
            }
            writeln!(f)?;
        }
        if let Some(total) = self.total_rating {
            write!(f, "  Total Rating: {:.1}", total)?;
            if let Some(count) = self.total_rating_count {
                write!(f, " ({} combined)", count)?;
            }
            writeln!(f)?;
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
        }
        if let Some(ts) = self.first_release_date {
            if let Some(date) = format_timestamp_pretty(Some(ts)) {
                writeln!(f, "  First Released: {}", date)?;
            }
        }
        if let Some(ts) = self.created_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Added to IGDB: {}", date)?;
            }
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Last Updated: {}", date)?;
            }
        }
        if let Some(ref cover) = self.cover {
            if let Some(ref img_id) = cover.image_id {
                writeln!(
                    f,
                    "  Cover: //images.igdb.com/igdb/image/upload/t_cover_big/{}.jpg",
                    img_id
                )?;
            }
        }
        if let Some(ref genres) = self.genres {
            let names: Vec<&str> = genres.iter().filter_map(|g| g.name.as_deref()).collect();
            if !names.is_empty() {
                writeln!(f, "  Genres: {}", names.join(", "))?;
            } else {
                writeln!(f, "  Genres: {} (IDs only, not expanded)", genres.len())?;
            }
        }
        if let Some(ref platforms) = self.platforms {
            let names: Vec<&str> = platforms.iter().filter_map(|p| p.name.as_deref()).collect();
            if !names.is_empty() {
                writeln!(f, "  Platforms: {}", names.join(", "))?;
            } else {
                writeln!(
                    f,
                    "  Platforms: {} (IDs only, not expanded)",
                    platforms.len()
                )?;
            }
        }
        if let Some(ref modes) = self.game_modes {
            let names: Vec<&str> = modes.iter().filter_map(|m| m.name.as_deref()).collect();
            if !names.is_empty() {
                writeln!(f, "  Game Modes: {}", names.join(", "))?;
            }
        }
        if let Some(ref themes) = self.themes {
            let names: Vec<&str> = themes.iter().filter_map(|t| t.name.as_deref()).collect();
            if !names.is_empty() {
                writeln!(f, "  Themes: {}", names.join(", "))?;
            }
        }
        if let Some(ref perspectives) = self.player_perspectives {
            let names: Vec<&str> = perspectives
                .iter()
                .filter_map(|p| p.name.as_deref())
                .collect();
            if !names.is_empty() {
                writeln!(f, "  Perspectives: {}", names.join(", "))?;
            }
        }
        if let Some(ref keywords) = self.keywords {
            let names: Vec<&str> = keywords.iter().filter_map(|k| k.name.as_deref()).collect();
            if !names.is_empty() {
                writeln!(f, "  Keywords: {}", names.join(", "))?;
            }
        }
        if let Some(ref companies) = self.involved_companies {
            let dev_names: Vec<String> = companies
                .iter()
                .filter(|ic| ic.is_developer())
                .filter_map(|ic| ic.company_name().map(String::from))
                .collect();
            if !dev_names.is_empty() {
                writeln!(f, "  Developers: {}", dev_names.join(", "))?;
            }
            let pub_names: Vec<String> = companies
                .iter()
                .filter(|ic| ic.is_publisher())
                .filter_map(|ic| ic.company_name().map(String::from))
                .collect();
            if !pub_names.is_empty() {
                writeln!(f, "  Publishers: {}", pub_names.join(", "))?;
            }
        }
        if let Some(ref dates) = self.release_dates {
            let mut formatted: Vec<String> = dates.iter().map(|rd| rd.display_date()).collect();
            if !formatted.is_empty() {
                formatted.sort();
                formatted.dedup();
                writeln!(f, "  Release Dates: {}", formatted.join(", "))?;
            }
        }
        if let Some(ref screenshots) = self.screenshots {
            if !screenshots.is_empty() {
                let has_images = screenshots.iter().any(|s| s.image_id.is_some());
                if has_images {
                    writeln!(f, "  Screenshots: {} (expanded)", screenshots.len())?;
                } else {
                    writeln!(f, "  Screenshots: {} (IDs only)", screenshots.len())?;
                }
            }
        }
        if let Some(ref age_ratings) = self.age_ratings {
            if !age_ratings.is_empty() {
                writeln!(f, "  Age Ratings:")?;
                for ar in age_ratings {
                    let org = ar.organization.as_ref().map(|o| format!("org:{}", o));
                    let rat = ar.rating_category.as_ref().map(|r| format!("cat:{}", r));

                    write!(f, "    {}", org.as_deref().unwrap_or("Unknown"))?;
                    if let Some(ref r) = rat {
                        write!(f, ": {}", r)?;
                    }
                    if let Some(ref syn) = ar.synopsis {
                        let short = if syn.len() > 60 {
                            format!("{}...", &syn[..60])
                        } else {
                            syn.clone()
                        };
                        write!(f, " - {}", short)?;
                    }
                    writeln!(f)?;
                }
            }
        }
        if let Some(ref version_title) = self.version_title {
            writeln!(f, "  Edition: {}", version_title)?;
        }

        Ok(())
    }
}
