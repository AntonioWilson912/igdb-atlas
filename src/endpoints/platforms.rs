//! Platform Endpoints
//!
//! Fluent interfaces for IGDB platform-related endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`PlatformsEndpoint`] | `/platforms` | [`Platform`] |
//! | [`PlatformFamiliesEndpoint`] | `/platform_families` | [`PlatformFamily`] |
//! | [`PlatformTypesEndpoint`] | `/platform_types` | [`PlatformType`] |
//! | [`PlatformVersionsEndpoint`] | `/platform_versions` | [`PlatformVersion`] |
//! | [`PlatformVersionCompaniesEndpoint`] | `/platform_version_companies` | [`PlatformVersionCompany`] |
//! | [`PlatformVersionReleaseDatesEndpoint`] | `/platform_version_release_dates` | [`PlatformVersionReleaseDate`] |

use crate::models::platforms::{
    Platform, PlatformFamily, PlatformType, PlatformVersion, PlatformVersionCompany,
    PlatformVersionReleaseDate,
};

define_endpoint! {
    /// `/platforms` - searchable.
    pub struct PlatformsEndpoint => "platforms", Platform, searchable
}

define_endpoint! {
    /// `/platform_families` - not searchable.
    pub struct PlatformFamiliesEndpoint => "platform_families", PlatformFamily, name_filterable
}

define_endpoint! {
    /// `/platform_types` - not searchable.
    pub struct PlatformTypesEndpoint => "platform_types", PlatformType, name_filterable
}

define_endpoint! {
    /// `/platform_versions` - not searchable.
    pub struct PlatformVersionsEndpoint => "platform_versions", PlatformVersion, name_filterable
}

define_endpoint! {
    /// `/platform_version_companies` - not searchable.
    pub struct PlatformVersionCompaniesEndpoint => "platform_version_companies", PlatformVersionCompany
}

define_endpoint! {
    /// `/platform_version_release_dates` - not searchable.
    pub struct PlatformVersionReleaseDatesEndpoint => "platform_version_release_dates", PlatformVersionReleaseDate
}
