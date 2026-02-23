//! # Event Model
//!
//! Represents an industry event from the IGDB v4 `/events` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::events::Event;
//!
//! let json = r#"{
//!     "id": 200,
//!     "name": "Nintendo Direct E3 2021",
//!     "slug": "nintendo-direct-e3-2021",
//!     "start_date": 1623772800
//! }"#;
//!
//! let event: Event = serde_json::from_str(json).unwrap();
//! assert_eq!(event.name, Some("Nintendo Direct E3 2021".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    Game,
    models::{
        events::EventNetwork,
        games::GameVideo,
        id_or_object::{FromId, deserialize_id_or_object, deserialize_id_or_object_vec},
        imagery::EventLogo,
        timestamp::{format_timestamp, format_timestamp_pretty},
    },
};

/// A game development or publishing event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// ISO 3166-1 numeric country code.
    #[serde(default)]
    pub country: Option<u16>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Free-text description of the event.
    #[serde(default)]
    pub description: Option<String>,

    /// End time of the event in UTC.
    #[serde(default)]
    pub end_time: Option<i64>,

    /// Logo of the event.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub event_logo: Option<EventLogo>,

    /// URLs associated with the event.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub event_networks: Option<Vec<EventNetwork>>,

    /// Games featured in the event.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub games: Option<Vec<Game>>,

    /// URL to the livestream of the event.
    #[serde(default)]
    pub live_stream_url: Option<String>,

    /// The name of the event.
    #[serde(default)]
    pub name: Option<String>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of when the event was founded.
    #[serde(default)]
    pub start_date: Option<i64>,

    /// Timezone the event is in.
    #[serde(default)]
    pub time_zone: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// Trailers featured in the event.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub videos: Option<Vec<GameVideo>>,
}

impl Event {
    /// Returns the event name or `"Unknown Event"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Event")
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            country: None,
            created_at: None,
            description: None,
            end_time: None,
            event_logo: None,
            event_networks: None,
            games: None,
            live_stream_url: None,
            name: None,
            slug: None,
            start_date: None,
            time_zone: None,
            updated_at: None,
            videos: None,
        }
    }
}

impl FromId for Event {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Event [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref desc) = self.description {
            let truncated = if desc.len() > 200 {
                format!("{}...", &desc[..200])
            } else {
                desc.clone()
            };
            writeln!(f, "  Description: {}", truncated)?;
        }
        if let Some(ts) = self.start_date {
            if let Some(date) = format_timestamp_pretty(Some(ts)) {
                writeln!(f, "  Start Date: {}", date)?;
            }
        }
        if let Some(ref url) = self.live_stream_url {
            writeln!(f, "  Livestream URL: {}", url)?;
        }
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
