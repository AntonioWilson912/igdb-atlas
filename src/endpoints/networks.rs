//! # Networks Endpoint
//!
//! Fluent interface for the IGDB `/network_types` endpoint.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`NetworkTypesEndpoint`] | `/network_types` | [`NetworkType`] |

use crate::models::networks::NetworkType;

define_endpoint! {
    /// `/network_types` - not searchable.
    pub struct NetworkTypesEndpoint => "network_types", NetworkType, name_filterable
}
