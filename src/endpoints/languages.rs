//! Language Endpoints
//!
//! Fluent interfaces for the IGDB language endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`LanguagesEndpoint`] | `/languages` | [`Language`] |
//! | [`LanguageSupportsEndpoint`] | `/language_supports` | [`LanguageSupport`] |
//! | [`LanguageSupportTypesEndpoint`] | `/language_support_types` | [`LanguageSupportType`] |

use crate::models::languages::{Language, LanguageSupport, LanguageSupportType};

define_endpoint! {
    /// `/languages` - not searchable.
    pub struct LanguagesEndpoint => "languages", Language, name_filterable
}

define_endpoint! {
    /// `/language_supports` - not searchable.
    pub struct LanguageSupportsEndpoint => "language_supports", LanguageSupport
}

define_endpoint! {
    /// `/language_support_types` - not searchable.
    pub struct LanguageSupportTypesEndpoint => "language_support_types", LanguageSupportType, name_filterable
}
