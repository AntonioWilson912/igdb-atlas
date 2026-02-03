//! # Age Rating Models
//!
//! Models for age / content rating data from the IGDB API.
//!
//! | Module | Endpoint | Notes |
//! |--------|----------|-------|
//! | [`age_rating`] | `/age_ratings` | Main model |
//! | [`age_rating_category`] | `/age_rating_categories` | Rating category reference |
//! | [`age_rating_content_description_type`] | `/age_rating_content_description_types` | Content-description type reference |
//! | [`age_rating_content_description_v2`] | `/age_rating_content_descriptions_v2` | Per-rating content flags |
//! | [`age_rating_organization`] | `/age_rating_organizations` | Rating organization reference |

pub mod age_rating;
pub mod age_rating_category;
pub mod age_rating_content_description_type;
pub mod age_rating_content_description_v2;
pub mod age_rating_organization;

pub use age_rating::AgeRating;
pub use age_rating_category::AgeRatingCategory;
pub use age_rating_content_description_type::AgeRatingContentDescriptionType;
pub use age_rating_content_description_v2::AgeRatingContentDescriptionV2;
pub use age_rating_organization::AgeRatingOrganization;
