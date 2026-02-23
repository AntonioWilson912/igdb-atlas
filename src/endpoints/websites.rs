//! Website Endpoints
//!
//! Fluent interfaces for IGDB website endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`CompanyWebsitesEndpoint`] | `/company_websites` | [`CompanyWebsite`] |
//! | [`PlatformWebsitesEndpoint`] | `/platform_websites` | [`PlatformWebsite`] |
//! | [`WebsitesEndpoint`] | `/websites` | [`Website`] |
//! | [`WebsiteTypesEndpoint`] | `/website_types` | [`WebsiteType`] |

use crate::models::websites::{CompanyWebsite, PlatformWebsite, Website, WebsiteType};

define_endpoint! {
    /// `/company_websites` - not searchable.
    pub struct CompanyWebsitesEndpoint => "company_websites", CompanyWebsite
}

define_endpoint! {
    /// `/platform_websites` - not searchable.
    pub struct PlatformWebsitesEndpoint => "platform_websites", PlatformWebsite
}

define_endpoint! {
    /// `/websites` - not searchable.
    pub struct WebsitesEndpoint => "websites", Website
}

define_endpoint! {
    /// `/website_types` - not searchable.
    pub struct WebsiteTypesEndpoint => "website_types", WebsiteType
}
