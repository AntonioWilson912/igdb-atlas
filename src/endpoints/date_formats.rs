//! # Date Formats Endpoint
//!
//! Fluent interface for the IGDB `/date_formats` endpoint.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`DateFormatsEndpoint`] | `/date_formats` | [`DateFormat`] |

use crate::models::date_formats::DateFormat;

define_endpoint! {
    /// `/date_formats` - not searchable.
    pub struct DateFormatsEndpoint => "date_formats", DateFormat
}
