//! # Company Models
//!
//! Models for game companies (developers, publishers, etc.).
//!
//! //! | Module | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`company`] | `/companies` | The primary [`Company`] model |
//! | [`company_size`] | `/company_sizes` | The size of a company |
//! | [`company_status`] | `/company_statuses` | The status of a company |
//! | [`company_type`] | `/company_types` | The type of a company |
//! | [`company_type_history`] | `/company_type_histories` | The history of a company type |
//!
//! Company logos live in [`crate::models::imagery::CompanyLogo`].
//! Company websites live in [`crate::models::websites::CompanyWebsite`]

pub mod company;
pub mod company_size;
pub mod company_status;
pub mod company_type;
pub mod company_type_history;

pub use company::Company;
pub use company_size::CompanySize;
pub use company_status::CompanyStatus;
pub use company_type::CompanyType;
pub use company_type_history::CompanyTypeHistory;
