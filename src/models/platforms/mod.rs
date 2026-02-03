//! # Platform Models
//!
//! Models for gaming platforms and related entities.
//!
//! - [`Platform`] - The primary platform model
//! - [`PlatformFamily`] - Groups of related platforms (e.g. PlayStation family)
//! - [`PlatformType`] - Types of platforms
//! - [`PlatformVersion`] - Specs about a platform
//! - [`PlatformVersionCompany`] - A platform developer
//! - [`PlatformVersionReleaseDate`] - Release dates for platforms
//!
//! Platform logo images live in [`crate::models::imagery::PlatformLogo`].

pub mod platform;
pub mod platform_family;
pub mod platform_type;
pub mod platform_version;
pub mod platform_version_company;
pub mod platform_version_release_date;

pub use platform::Platform;
pub use platform_family::PlatformFamily;
pub use platform_type::PlatformType;
pub use platform_version::PlatformVersion;
pub use platform_version_company::PlatformVersionCompany;
pub use platform_version_release_date::PlatformVersionReleaseDate;
