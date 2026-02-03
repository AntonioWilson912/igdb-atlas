//! # Event Models
//!
//! Models for events (networks, etc.).
//!
//! //! | Module | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`event`] | `/events` | The primary [`Event`] model |
//! | [`event_network`] | `/event_networks` | Network for the event |
//!
//! Event logos live in [`crate::models::imagery::EventLogo`].

pub mod event;
pub mod event_network;

pub use event::Event;
pub use event_network::EventNetwork;
