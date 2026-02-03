//! # Network Type Model
//!
//! Represents an event-related social network from the IGDB v4 `/network_types` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::networks::NetworkType;
//!
//! let json = r#"{
//!     "id": 4,
//!     "name": "Twitter",
//!     "event_networks": [72, 74]
//! }"#;
//! let network_type: NetworkType = serde_json::from_str(json).unwrap();
//! assert_eq!(network_type.display_name(), "Twitter");
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    events::EventNetwork,
    id_or_object::{FromId, deserialize_id_or_object_vec},
    timestamp::format_timestamp,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkType {
    /// Unique network type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// URLs associated with the event type.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub event_networks: Option<Vec<EventNetwork>>,

    /// The name of the network type.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl NetworkType {
    /// Returns the network type name or `"Unknown Network Type"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Network Type")
    }
}

impl Default for NetworkType {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            event_networks: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for NetworkType {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NetworkType [{}]", self.id)?;
        if let Some(ref name) = self.name {
            write!(f, " name={}", name)?;
        }
        if let Some(ref event_networks) = self.event_networks {
            writeln!(
                f,
                "  Event Networks: {} (IDs only, not expanded)",
                event_networks.len()
            )?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " (updated {})", d)?;
            }
        }
        writeln!(f)
    }
}
