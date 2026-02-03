//! # Website Models
//!
//! Models for website URLs associated with games, companies, and platforms.
//!
//! ## Sub-modules
//!
//! - [`company_website`] - Company website URLs (`/company_websites`)
//! - [`platform_website`] - Platform website URLs (`/platform_websites`)
//! - [`website`] - Website URLs (`/websites`)
//! - [`website_type`] - Website type reference table (`/website_types`)

pub mod company_website;
pub mod platform_website;
pub mod website;
pub mod website_type;

pub use company_website::CompanyWebsite;
pub use platform_website::PlatformWebsite;
pub use website::Website;
pub use website_type::WebsiteType;
