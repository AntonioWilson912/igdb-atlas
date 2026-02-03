//! # Event Network Model
//!
//! Represents an event network from the IGDB v4
//! `/event_networks` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::events::EventNetwork;
//!
//! let json = r#"{
//!     "id": 160877,
//!     "url": "https://awards.cesa.or.jp/en/"
//! }"#;
//!
//! let event_network: EventNetwork = serde_json::from_str(json).unwrap();
//! assert_eq!(event_network.url, Some("https://awards.cesa.or.jp/en/".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Event, NetworkType,
    models::{
        id_or_object::{FromId, deserialize_id_or_object},
        timestamp::format_timestamp,
    },
};

/// An event network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNetwork {
    /// Unique event network identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The event associated with this URL.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub event: Option<Event>,

    /// The type of network.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub network_type: Option<NetworkType>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The website address (URL) of the item.
    #[serde(default)]
    pub url: Option<String>,
}

impl EventNetwork {}

impl Default for EventNetwork {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            event: None,
            network_type: None,
            updated_at: None,
            url: None,
        }
    }
}

impl FromId for EventNetwork {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for EventNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "EventNetwork [{}]", self.id)?;
        if let Some(ts) = self.created_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Added: {}", date)?;
            }
        }
        if let Some(ts) = self.updated_at {
            if let Some(date) = format_timestamp(Some(ts)) {
                writeln!(f, "  Updated: {}", date)?;
            }
        }
        Ok(())
    }
}
