//! # Companies Endpoint
//!
//! Fluent interfaces for IGDB company-related endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`CompaniesEndpoint`] | `/companies` | [`Company`] |
//! | [`CompanyStatusesEndpoint`] | `/company_statuses` | [`CompanyStatus`] |

use crate::models::companies::{Company, CompanyStatus};

define_endpoint! {
    /// `/companies` - searchable.
    pub struct CompaniesEndpoint => "companies", Company, searchable
}

define_endpoint! {
    /// `/company_statuses` - not searchable.
    pub struct CompanyStatusesEndpoint => "company_statuses", CompanyStatus, name_filterable
}
