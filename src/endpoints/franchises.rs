//! # Franchises Endpoint
//!
//! Fluent interface for the IGDB `/franchises` endpoint.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`FranchisesEndpoint`] | `/franchises` | [`Franchise`] |

use crate::models::franchises::Franchise;

define_endpoint! {
    /// `/franchises` - not searchable.
    pub struct FranchisesEndpoint => "franchises", Franchise, name_filterable
}
