//! # Company Models
//!
//! Models for game companies (developers, publishers, etc.).
//!
//! //! | Module | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`company`] | `/companies` | The primary [`Company`] model |
//! | [`company_status`] | `/company_statuses` | The status of a company |
//!
//! Company logos live in [`crate::models::imagery::CompanyLogo`].
//! Company websites live in [`crate::models::websites::CompanyWebsite`]

pub mod company;
pub mod company_status;

pub use company::Company;
pub use company_status::CompanyStatus;
