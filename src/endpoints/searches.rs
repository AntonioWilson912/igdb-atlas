//! # Search Endpoint
//!
//! Fluent interface for the IGDB `/search` endpoint.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`SearchEndpoint`] | `/search` | [`Search`] |

use crate::models::searches::Search;

define_endpoint! {
    /// `/search`.
    pub struct SearchEndpoint => "search", Search, name_filterable
}
