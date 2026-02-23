//! # Events Endpoint
//!
//! Fluent interfaces for IGDB event interfaces.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`EventsEndpoint`] | `/events` | [`Event`] |
//! | [`EventNetworksEndpoint`] | `/event_networks` | [`EventNetwork`] |

use crate::models::events::{Event, EventNetwork};

define_endpoint! {
    /// `/events` - not searchable.
    pub struct EventsEndpoint => "events", Event
}

define_endpoint! {
    /// `/event_networks` - not searchable.
    pub struct EventNetworksEndpoint => "event_networks", EventNetwork
}
