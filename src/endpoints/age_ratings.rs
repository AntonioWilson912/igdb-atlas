//! # Age Rating Endpoints
//!
//! Fluent interfaces for all IGDB age-rating-related endpoints.
//!
//! ## Endpoints
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`AgeRatingsEndpoint`] | `/age_ratings` | [`AgeRating`] |
//! | [`AgeRatingCategoriesEndpoint`] | `/age_rating_categories` | [`AgeRatingCategory`] |
//! | [`AgeRatingContentDescriptionTypesEndpoint`] | `/age_rating_content_description_types` | [`AgeRatingContentDescriptionType`] |
//! | [`AgeRatingContentDescriptionsV2Endpoint`] | `/age_rating_content_descriptions_v2` | [`AgeRatingContentDescriptionV2`] |
//! | [`AgeRatingOrganizationsEndpoint`] | `/age_rating_organizations` | [`AgeRatingOrganization`] |

use crate::models::age_ratings::{
    AgeRating, AgeRatingCategory, AgeRatingContentDescriptionType, AgeRatingContentDescriptionV2,
    AgeRatingOrganization,
};

define_endpoint! {
    /// `/age_ratings` - not searchable.
    pub struct AgeRatingsEndpoint => "age_ratings", AgeRating
}

define_endpoint! {
    /// `/age_rating_categories` - not searchable.
    pub struct AgeRatingCategoriesEndpoint => "age_rating_categories", AgeRatingCategory
}

define_endpoint! {
    /// `/age_rating_content_description_types` - not searchable.
    pub struct AgeRatingContentDescriptionTypesEndpoint => "age_rating_content_description_types", AgeRatingContentDescriptionType
}

define_endpoint! {
    /// `/age_rating_content_descriptions_v2` - not searchable.
    pub struct AgeRatingContentDescriptionsV2Endpoint => "age_rating_content_descriptions_v2", AgeRatingContentDescriptionV2
}

define_endpoint! {
    /// `/age_rating_organizations` - not searchable.
    pub struct AgeRatingOrganizationsEndpoint => "age_rating_organizations", AgeRatingOrganization
}
