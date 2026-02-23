//! # Client Module
//!
//! The core HTTP client that orchestrates authentication, rate limiting,
//! and endpoint access for the IGDB API.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
//! │   TokenMgr   │<────│  IGDBClient  │────>│  RateLimiter │
//! │    (auth)    │     │  (orchestr.) │     │   (4 req/s)  │
//! └──────────────┘     └──────┬───────┘     └──────────────┘
//!                             │
//!                             │
//!                             v
//!                       ┌───────────┐
//!                       │ Endpoints │
//!                       └───────────┘
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use igdb_atlas::{IGDBClient, ClientConfig};
//! use igdb_atlas::endpoints::traits::{Endpoint, Searchable};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig::new("client_id", "client_secret");
//!     let client = IGDBClient::new(config).await.unwrap();
//!
//!     let games = client.games().search("Zelda").limit(3).execute().await.unwrap();
//!     println!("Found {} games", games.len());
//! }
//! ```

pub mod config;
pub mod rate_limiter;

use std::sync::Arc;

use crate::{
    auth::TokenManager,
    client::{config::ClientConfig, rate_limiter::RateLimiter},
    endpoints::{
        age_ratings::{
            AgeRatingCategoriesEndpoint, AgeRatingContentDescriptionTypesEndpoint,
            AgeRatingContentDescriptionsV2Endpoint, AgeRatingOrganizationsEndpoint,
            AgeRatingsEndpoint,
        },
        characters::{CharacterGendersEndpoint, CharacterSpeciesEndpoint, CharactersEndpoint},
        collections::{
            CollectionMembershipTypesEndpoint, CollectionMembershipsEndpoint,
            CollectionRelationTypesEndpoint, CollectionRelationsEndpoint, CollectionTypesEndpoint,
            CollectionsEndpoint,
        },
        companies::{
            CompaniesEndpoint, CompanySizesEndpoint, CompanyStatusesEndpoint,
            CompanyTypeHistoriesEndpoint, CompanyTypesEndpoint,
        },
        date_formats::DateFormatsEndpoint,
        events::{EventNetworksEndpoint, EventsEndpoint},
        franchises::FranchisesEndpoint,
        games::{
            AlternativeNamesEndpoint, ContentSafetyRatingDimensionsEndpoint,
            ContentSafetyRatingsEndpoint, ExternalGameSourcesEndpoint, ExternalGamesEndpoint,
            GameContentSafetyRatingsEndpoint, GameEnginesEndpoint, GameLocalizationsEndpoint,
            GameModesEndpoint, GameReleaseFormatsEndpoint, GameStatusesEndpoint,
            GameTimeToBeatsEndpoint, GameTypesEndpoint, GameVersionFeatureValuesEndpoint,
            GameVersionFeaturesEndpoint, GameVersionsEndpoint, GameVideosEndpoint, GamesEndpoint,
            GenresEndpoint, InvolvedCompaniesEndpoint, KeywordsEndpoint, MultiplayerModesEndpoint,
            PlayerPerspectivesEndpoint, PopularityPrimitivesEndpoint, PopularityTypesEndpoint,
            RegionsEndpoint, ReleaseDateRegionsEndpoint, ReleaseDateStatusesEndpoint,
            ReleaseDatesEndpoint,
        },
        imagery::{
            ArtworkTypesEndpoint, ArtworksEndpoint, CharacterMugShotsEndpoint,
            CompanyLogosEndpoint, CoversEndpoint, EventLogosEndpoint, GameEngineLogosEndpoint,
            PlatformLogosEndpoint, ScreenshotsEndpoint,
        },
        languages::{LanguageSupportTypesEndpoint, LanguageSupportsEndpoint, LanguagesEndpoint},
        networks::NetworkTypesEndpoint,
        platforms::{
            PlatformFamiliesEndpoint, PlatformTypesEndpoint, PlatformVersionCompaniesEndpoint,
            PlatformVersionReleaseDatesEndpoint, PlatformVersionsEndpoint, PlatformsEndpoint,
        },
        searches::SearchEndpoint,
        themes::ThemesEndpoint,
        websites::{
            CompanyWebsitesEndpoint, PlatformWebsitesEndpoint, WebsiteTypesEndpoint,
            WebsitesEndpoint,
        },
    },
    error::{IGDBError, Result},
};

/// The base URL for all IGDB API v4 requests.
pub const IGDB_BASE_URL: &str = "https://api.igdb.com/v4";

/// The main client for interacting with the IGDB API.
///
/// Provides access to all endpoint handlers and manages the underlying
/// HTTP client, authentication, and rate limiting.
///
/// # Cloning
///
/// `IGDBClient` is cheap to clone - all internal state is reference-counted.
/// This makes it easy to share across tasks without `Arc<Mutex<_>>`.
///
/// # Examples
///
/// ```rust,no_run
/// use igdb_atlas::{IGDBClient, ClientConfig};
/// use igdb_atlas::endpoints::traits::Searchable;
///
/// async fn example() {
///     let config = ClientConfig::new("id", "secret");
///     let client = IGDBClient::new(config).await.unwrap();
///
///     // Clone freely for concurrent use
///     let client2 = client.clone();
///     tokio::spawn(async move {
///         let _ = client2.games().search("test").execute().await;
///     });
/// }
/// ```
#[derive(Clone, Debug)]
pub struct IGDBClient {
    base_url: String,
    client_id: String,
    http_client: reqwest::Client,
    rate_limiter: Arc<RateLimiter>,
    token_manager: Arc<TokenManager>,
}

impl IGDBClient {
    /// Creates a new `IGDBClient` from the given configuration.
    ///
    /// This constructor validates the configuration and initializes
    /// all internal components. The OAuth token is fetched lazily
    /// on the first actual API request.
    ///
    /// # Errors
    ///
    /// Returns [`IGDBError::InvalidConfiguration`] if required fields are missing.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::{IGDBClient, ClientConfig};
    ///
    /// async fn example() {
    ///     let config = ClientConfig::new("my_id", "my_secret");
    ///     let client = IGDBClient::new(config).await.unwrap();
    /// }
    /// ```
    pub async fn new(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        let rate_limiter = RateLimiter::new(config.rate_limit_rps, config.max_backoff_attempts);

        let token_manager =
            TokenManager::new(config.client_id.clone(), config.client_secret.clone());

        Ok(Self {
            base_url: config.base_url,
            client_id: config.client_id,
            http_client: reqwest::Client::new(),
            rate_limiter: Arc::new(rate_limiter),
            token_manager: Arc::new(token_manager),
        })
    }

    /// Executes a raw Apicalypse query against the specified endpoint path.
    ///
    /// This is the low-level method used by all endpoint handlers.
    /// It handles authentication, rate limiting, and error translation.
    ///
    /// # Generic Parameters
    ///
    /// - `T`: The response model type. Must implement `serde::Deserialize`.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The API endpoint path (e.g., `"games"`, `"platforms"`)
    /// - `query`: The Apicalypse query string (e.g., `"search \"zelda\"; limit 5;"`)
    ///
    /// # Errors
    ///
    /// - [`IGDBError::RateLimited`] if max backoff attempts are exhausted
    /// - [`IGDBError::ApiError`] for non-success HTTP responses
    /// - [`IGDBError::DeserializationError`] if the response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::{IGDBClient, ClientConfig};
    /// use igdb_atlas::models::games::Game;
    ///
    /// async fn example() {
    ///     let config = ClientConfig::new("id", "secret");
    ///     let client = IGDBClient::new(config).await.unwrap();
    ///
    ///     let games: Vec<Game> = client
    ///         .execute_query("games", "search \"zelda\"; limit 5;")
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn execute_query<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &str,
    ) -> Result<T> {
        self.execute_query_internal(endpoint, query).await
    }

    /// Internal method that executes a query and handles the full response lifecycle.
    async fn execute_query_internal<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &str,
    ) -> Result<T> {
        let token = self.token_manager.get_valid_token().await?;
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self
            .rate_limiter
            .execute_with_backoff(|| {
                self.http_client
                    .post(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .header("Client-ID", &self.client_id)
                    .header("Content-Type", "text/plain")
                    .body(query.to_string())
                    .send()
            })
            .await?;

        let status = response.status().as_u16();

        if status == 200 {
            let bytes = response.bytes().await.map_err(|e| IGDBError::ApiError {
                status: 0,
                message: format!("Failed to read response body: {}", e),
            })?;

            serde_json::from_slice(&bytes).map_err(|e| IGDBError::DeserializationError(e))
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(IGDBError::ApiError {
                status,
                message: body,
            })
        }
    }

    /// Returns a handle to the `/age_ratings` endpoint. Not searchable.
    pub fn age_ratings(&self) -> AgeRatingsEndpoint {
        AgeRatingsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/age_categories` endpoint. Not searchable.
    pub fn age_categories(&self) -> AgeRatingCategoriesEndpoint {
        AgeRatingCategoriesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/age_rating_content_description_type` endpoint. Not searchable.
    pub fn age_rating_content_description_types(&self) -> AgeRatingContentDescriptionTypesEndpoint {
        AgeRatingContentDescriptionTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/age_rating_content_descriptions_v2` endpoint. Not searchable.
    pub fn age_rating_content_descriptions_v2(&self) -> AgeRatingContentDescriptionsV2Endpoint {
        AgeRatingContentDescriptionsV2Endpoint::new(self.clone())
    }

    /// Returns a handle to the `/age_rating_organizations` endpoint. Not searchable.
    pub fn age_rating_organizations(&self) -> AgeRatingOrganizationsEndpoint {
        AgeRatingOrganizationsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/alternative_names` endpoint. Not searchable.
    pub fn alternative_names(&self) -> AlternativeNamesEndpoint {
        AlternativeNamesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/artworks` endpoint. Not searchable.
    pub fn artworks(&self) -> ArtworksEndpoint {
        ArtworksEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/artwork_types` endpoint. Name filterable.
    pub fn artwork_types(&self) -> ArtworkTypesEndpoint {
        ArtworkTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/characters` endpoint. Searchable.
    pub fn characters(&self) -> CharactersEndpoint {
        CharactersEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/character_genders` endpoint. Name filterable.
    pub fn character_genders(&self) -> CharacterGendersEndpoint {
        CharacterGendersEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/character_mug_shots` endpoint. Not searchable.
    pub fn character_mug_shots(&self) -> CharacterMugShotsEndpoint {
        CharacterMugShotsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/character_species` endpoint. Name filterable.
    pub fn character_species(&self) -> CharacterSpeciesEndpoint {
        CharacterSpeciesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/collections` endpoint. Searchable.
    pub fn collections(&self) -> CollectionsEndpoint {
        CollectionsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/collection_memberships` endpoint. Not searchable.
    pub fn collection_memberships(&self) -> CollectionMembershipsEndpoint {
        CollectionMembershipsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/collection_membership_types` endpoint. Name filterable.
    pub fn collection_membership_types(&self) -> CollectionMembershipTypesEndpoint {
        CollectionMembershipTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/collection_relations` endpoint. Not searchable.
    pub fn collection_relations(&self) -> CollectionRelationsEndpoint {
        CollectionRelationsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/collection_relation_types` endpoint. Name filterable.
    pub fn collection_relation_types(&self) -> CollectionRelationTypesEndpoint {
        CollectionRelationTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/collection_types` endpoint. Name filterable.
    pub fn collection_types(&self) -> CollectionTypesEndpoint {
        CollectionTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/companies` endpoint. Searchable.
    pub fn companies(&self) -> CompaniesEndpoint {
        CompaniesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/company_logos` endpoint. Not searchable.
    pub fn company_logos(&self) -> CompanyLogosEndpoint {
        CompanyLogosEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/company_sizes` endpoint. Name filterable.
    pub fn company_sizes(&self) -> CompanySizesEndpoint {
        CompanySizesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/company_status` endpoint. Name filterable.
    pub fn company_statuses(&self) -> CompanyStatusesEndpoint {
        CompanyStatusesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/company_types` endpoint. Name filterable.
    pub fn company_types(&self) -> CompanyTypesEndpoint {
        CompanyTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/company_type_histories` endpoint. Not searchable.
    pub fn company_type_histories(&self) -> CompanyTypeHistoriesEndpoint {
        CompanyTypeHistoriesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/company_websites` endpoint. Not searchable.
    pub fn company_websites(&self) -> CompanyWebsitesEndpoint {
        CompanyWebsitesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/content_safety_ratings` endpoint. Name filterable.
    ///
    /// > **Note:** Requires elevated API access. Returns `403` on free tier.
    pub fn content_safety_ratings(&self) -> ContentSafetyRatingsEndpoint {
        ContentSafetyRatingsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/content_safety_rating_dimensions` endpoint. Not searchable.
    ///
    /// > **Note:** Requires elevated API access. Returns `403` on free tier.
    pub fn content_safety_rating_dimensions(&self) -> ContentSafetyRatingDimensionsEndpoint {
        ContentSafetyRatingDimensionsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/covers` endpoint. Not searchable.
    pub fn covers(&self) -> CoversEndpoint {
        CoversEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/date_formats` endpoint. Not searchable.
    pub fn date_formats(&self) -> DateFormatsEndpoint {
        DateFormatsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/events` endpoint. Name filterable.
    pub fn events(&self) -> EventsEndpoint {
        EventsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/event_logos` endpoint. Not searchable.
    pub fn event_logos(&self) -> EventLogosEndpoint {
        EventLogosEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/event_networks` endpoint. Not searchable.
    pub fn event_networks(&self) -> EventNetworksEndpoint {
        EventNetworksEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/external_games` endpoint. Not searchable.
    pub fn external_games(&self) -> ExternalGamesEndpoint {
        ExternalGamesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/external_game_sources` endpoint. Not searchable.
    pub fn external_game_sources(&self) -> ExternalGameSourcesEndpoint {
        ExternalGameSourcesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/franchises` endpoint. Name filterable.
    pub fn franchises(&self) -> FranchisesEndpoint {
        FranchisesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/games` endpoint. Searchable.
    pub fn games(&self) -> GamesEndpoint {
        GamesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_content_safety_ratings` endpoint. Not searchable.
    ///
    /// > **Note:** Requires elevated API access. Returns `403` on free tier.
    pub fn game_content_safety_ratings(&self) -> GameContentSafetyRatingsEndpoint {
        GameContentSafetyRatingsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_engines` endpoint. Not searchable.
    pub fn game_engines(&self) -> GameEnginesEndpoint {
        GameEnginesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_engine_logos` endpoint. Not searchable.
    pub fn game_engine_logos(&self) -> GameEngineLogosEndpoint {
        GameEngineLogosEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_localizations` endpoint. Name filterable.
    pub fn game_localizations(&self) -> GameLocalizationsEndpoint {
        GameLocalizationsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_modes` endpoint. Name filterable.
    pub fn game_modes(&self) -> GameModesEndpoint {
        GameModesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_release_formats` endpoint. Not searchable.
    pub fn game_release_formats(&self) -> GameReleaseFormatsEndpoint {
        GameReleaseFormatsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_statuses` endpoint. Not searchable.
    pub fn game_statuses(&self) -> GameStatusesEndpoint {
        GameStatusesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_time_to_beat` endpoint. Not searchable.
    pub fn game_time_to_beats(&self) -> GameTimeToBeatsEndpoint {
        GameTimeToBeatsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_types` endpoint. Not searchable.
    pub fn game_types(&self) -> GameTypesEndpoint {
        GameTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_versions` endpoint. Not searchable.
    pub fn game_versions(&self) -> GameVersionsEndpoint {
        GameVersionsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_version_features` endpoint. Not searchable.
    pub fn game_version_features(&self) -> GameVersionFeaturesEndpoint {
        GameVersionFeaturesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_version_feature_values` endpoint. Not searchable.
    pub fn game_version_feature_values(&self) -> GameVersionFeatureValuesEndpoint {
        GameVersionFeatureValuesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/game_videos` endpoint. Name filterable.
    pub fn game_videos(&self) -> GameVideosEndpoint {
        GameVideosEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/genres` endpoint. Searchable.
    pub fn genres(&self) -> GenresEndpoint {
        GenresEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/involved_companies` endpoint. Not searchable.
    pub fn involved_companies(&self) -> InvolvedCompaniesEndpoint {
        InvolvedCompaniesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/keywords` endpoint. Not searchable.
    pub fn keywords(&self) -> KeywordsEndpoint {
        KeywordsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/languages` endpoint. Name filterable.
    pub fn languages(&self) -> LanguagesEndpoint {
        LanguagesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/language_supports` endpoint. Not searchable.
    pub fn language_supports(&self) -> LanguageSupportsEndpoint {
        LanguageSupportsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/language_support_types` endpoint. Name filterable.
    pub fn language_support_types(&self) -> LanguageSupportTypesEndpoint {
        LanguageSupportTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/multiplayer_modes` endpoint. Not searchable.
    pub fn multiplayer_modes(&self) -> MultiplayerModesEndpoint {
        MultiplayerModesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/network_types` endpoint. Name filterable.
    pub fn network_types(&self) -> NetworkTypesEndpoint {
        NetworkTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platforms` endpoint. Searchable.
    pub fn platforms(&self) -> PlatformsEndpoint {
        PlatformsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platform_families` endpoint. Name filterable.
    pub fn platform_families(&self) -> PlatformFamiliesEndpoint {
        PlatformFamiliesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platform_logos` endpoint. Not searchable.
    pub fn platform_logos(&self) -> PlatformLogosEndpoint {
        PlatformLogosEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platform_types` endpoint. Name filterable.
    pub fn platform_types(&self) -> PlatformTypesEndpoint {
        PlatformTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platform_versions` endpoint. Name filterable.
    pub fn platform_versions(&self) -> PlatformVersionsEndpoint {
        PlatformVersionsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platform_version_companies` endpoint. Not searchable.
    pub fn platform_version_companies(&self) -> PlatformVersionCompaniesEndpoint {
        PlatformVersionCompaniesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platform_version_release_dates` endpoint. Not searchable.
    pub fn platform_version_release_dates(&self) -> PlatformVersionReleaseDatesEndpoint {
        PlatformVersionReleaseDatesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/platform_websites` endpoint. Not searchable.
    pub fn platform_websites(&self) -> PlatformWebsitesEndpoint {
        PlatformWebsitesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/player_perspectives` endpoint. Name filterable.
    pub fn player_perspectives(&self) -> PlayerPerspectivesEndpoint {
        PlayerPerspectivesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/popularity_primitives` endpoint. Not searchable.
    pub fn popularity_primitives(&self) -> PopularityPrimitivesEndpoint {
        PopularityPrimitivesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/popularity_types` endpoint. Name filterable.
    pub fn popularity_types(&self) -> PopularityTypesEndpoint {
        PopularityTypesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/regions` endpoint. Name filterable.
    pub fn regions(&self) -> RegionsEndpoint {
        RegionsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/release_dates` endpoint. Not searchable.
    pub fn release_dates(&self) -> ReleaseDatesEndpoint {
        ReleaseDatesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/release_date_regions` endpoint. Not searchable.
    pub fn release_date_regions(&self) -> ReleaseDateRegionsEndpoint {
        ReleaseDateRegionsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/release_date_statuses` endpoint. Name filterable.
    pub fn release_date_statuses(&self) -> ReleaseDateStatusesEndpoint {
        ReleaseDateStatusesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/screenshots` endpoint. Not searchable.
    pub fn screenshots(&self) -> ScreenshotsEndpoint {
        ScreenshotsEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/search` endpoint. Name filterable.
    pub fn search(&self) -> SearchEndpoint {
        SearchEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/themes` endpoint. Searchable.
    pub fn themes(&self) -> ThemesEndpoint {
        ThemesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/websites` endpoint. Not searchable.
    pub fn websites(&self) -> WebsitesEndpoint {
        WebsitesEndpoint::new(self.clone())
    }

    /// Returns a handle to the `/website_types` endpoint. Not searchable.
    pub fn website_types(&self) -> WebsiteTypesEndpoint {
        WebsiteTypesEndpoint::new(self.clone())
    }
}
