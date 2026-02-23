//! # Companies Endpoint
//!
//! Fluent interfaces for IGDB company-related endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`CompaniesEndpoint`] | `/companies` | [`Company`] |
//! | [`CompanySizesEndpoint`] | `/company_sizes` | [`CompanySize`] |
//! | [`CompanyStatusesEndpoint`] | `/company_statuses` | [`CompanyStatus`] |
//! | [`CompanyTypesEndpoint`] | `/company_types` | [`CompanyType`] |
//! | [`CompanyTypeHistoriesEndpoint`] | `/company_type_histories` | [`CompanyTypeHistory`] |

use crate::models::companies::{
    Company, CompanySize, CompanyStatus, CompanyType, CompanyTypeHistory,
};

define_endpoint! {
    /// `/companies` - searchable.
    pub struct CompaniesEndpoint => "companies", Company, searchable
}

define_endpoint! {
    /// `/company_sizes` - searchable.
    pub struct CompanySizesEndpoint => "company_sizes", CompanySize
}

define_endpoint! {
    /// `/company_statuses` - not searchable.
    pub struct CompanyStatusesEndpoint => "company_statuses", CompanyStatus, name_filterable
}

define_endpoint! {
    /// `/company_types` - searchable.
    pub struct CompanyTypesEndpoint => "company_types", CompanyType
}

define_endpoint! {
    /// `/company_type_histories` - searchable.
    pub struct CompanyTypeHistoriesEndpoint => "company_type_histories", CompanyTypeHistory
}
