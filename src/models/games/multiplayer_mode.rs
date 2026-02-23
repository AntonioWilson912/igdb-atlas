//! # Multiplayer Mode Model
//!
//! Represents multiplayer mode data from the IGDB v4
//! `/multiplayer_modes` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::MultiplayerMode;
//!
//! let json = r#"{
//!     "id": 1,
//!     "game": 1942,
//!     "onlinecoop": true,
//!     "onlinecoopmax": 4,
//!     "onlinemax": 8,
//!     "splitscreen": false
//! }"#;
//!
//! let mode: MultiplayerMode = serde_json::from_str(json).unwrap();
//! assert!(mode.has_online());
//! assert_eq!(mode.onlinemax, Some(8));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    games::Game,
    id_or_object::{FromId, deserialize_id_or_object},
    platforms::Platform,
};

/// Multiplayer mode information for a game, optionally scoped to a
/// specific platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiplayerMode {
    /// Unique multiplayer mode identifier.
    pub id: u64,

    /// Whether the game supports campaign co-op.
    #[serde(default)]
    pub campaigncoop: Option<bool>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Whether the game supports drop in/out multiplayer.
    #[serde(default)]
    pub dropin: Option<bool>,

    /// The game this multiplayer mode is associated with.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub game: Option<Game>,

    /// Whether the game supports LAN co-op.
    #[serde(default)]
    pub lancoop: Option<bool>,

    /// Whether the game supports offline co-op.
    #[serde(default)]
    pub offlinecoop: Option<bool>,

    /// Maximum number of offline players in offline co-op.
    #[serde(default)]
    pub offlinecoopmax: Option<u32>,

    /// Maximum number of players in offline multiplayer.
    #[serde(default)]
    pub offlinemax: Option<u32>,

    /// Whether the game supports online co-op.
    #[serde(default)]
    pub onlinecoop: Option<bool>,

    /// Maximum number of online players in online co-op.
    #[serde(default)]
    pub onlinecoopmax: Option<u32>,

    /// Maximum number of players in online multiplayer.
    #[serde(default)]
    pub onlinemax: Option<u32>,

    /// The platform this multiplayer mode refers to.
    #[serde(default)]
    pub platform: Option<Platform>,

    /// Whether the game supports split-screen offline multiplayer.
    #[serde(default)]
    pub splitscreen: Option<bool>,

    /// Whether the game supports split-screen online multiplayer.
    #[serde(default)]
    pub splitscreenonline: Option<bool>,
}

impl MultiplayerMode {
    /// Returns `true` if any co-op mode is supported.
    pub fn has_coop(&self) -> bool {
        self.campaigncoop == Some(true)
            || self.lancoop == Some(true)
            || self.offlinecoop == Some(true)
            || self.onlinecoop == Some(true)
    }

    /// Returns `true` if any online mode is supported.
    pub fn has_online(&self) -> bool {
        self.onlinecoop == Some(true)
            || self.onlinemax.map(|n| n > 0) == Some(true)
            || self.onlinecoopmax.map(|n| n > 0) == Some(true)
            || self.splitscreenonline == Some(true)
    }

    /// Returns `true` if split-screen is supported (online or offline).
    pub fn has_split_screen(&self) -> bool {
        self.splitscreen == Some(true) || self.splitscreenonline == Some(true)
    }

    /// Returns the maximum player count across all modes.
    pub fn max_players(&self) -> Option<u32> {
        [
            self.offlinemax,
            self.offlinecoopmax,
            self.onlinemax,
            self.onlinecoopmax,
        ]
        .iter()
        .filter_map(|&v| v)
        .max()
    }
}

impl Default for MultiplayerMode {
    fn default() -> Self {
        Self {
            id: 0,
            campaigncoop: None,
            checksum: None,
            dropin: None,
            game: None,
            lancoop: None,
            offlinecoop: None,
            offlinecoopmax: None,
            offlinemax: None,
            onlinecoop: None,
            onlinecoopmax: None,
            onlinemax: None,
            platform: None,
            splitscreen: None,
            splitscreenonline: None,
        }
    }
}

impl FromId for MultiplayerMode {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for MultiplayerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MultiplayerMode [{}]", self.id)?;
        if let Some(ref game) = self.game {
            writeln!(f, "  Game ID: {}", game)?;
        }
        if let Some(ref platform) = self.platform {
            writeln!(f, "  Platform ID: {}", platform)?;
        }
        let mut modes = Vec::new();
        if self.campaigncoop == Some(true) {
            modes.push("Campaign Co-op");
        }
        if self.lancoop == Some(true) {
            modes.push("LAN Co-op");
        }
        if self.offlinecoop == Some(true) {
            modes.push("Offline Co-op");
        }
        if self.onlinecoop == Some(true) {
            modes.push("Online Co-op");
        }
        if self.splitscreen == Some(true) {
            modes.push("Split Screen");
        }
        if self.splitscreenonline == Some(true) {
            modes.push("Split Screen Online");
        }
        if self.dropin == Some(true) {
            modes.push("Drop-in/Drop-out");
        }
        if !modes.is_empty() {
            writeln!(f, "  Modes: {}", modes.join(", "))?;
        }
        if let Some(max) = self.onlinemax {
            writeln!(f, "  Online Max: {}", max)?;
        }
        if let Some(max) = self.offlinemax {
            writeln!(f, "  Offline Max: {}", max)?;
        }
        Ok(())
    }
}
