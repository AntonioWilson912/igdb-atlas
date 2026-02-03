//! # Common Models
//!
//! Lightweight reference types shared across multiple domain modules.
//!
//! Each struct carries only an `id` (and occasionally one or two name
//! fields). They are used when the API returns a nested reference -
//! expanded or bare-ID - without requiring the full domain model.
//!
//! | Type | Full model |
//! |------|------------|
//! | [`CompanyRef`] | [`crate::models::companies::Company`] |
//! | [`EventRef`] | [`crate::models::events::Event`] |
//! | [`GameRef`] | [`crate::models::games::Game`] |
//! | [`GameLocalizationRef`] | [`crate::models::game::GameLocalization`] |
//! | [`PlatformRef`] | [`crate::models::platforms::Platform`] |
//! | [`ThemeRef`] | [`crate::models::themes::Theme`] |

use serde::{Deserialize, Serialize};

use crate::models::id_or_object::FromId;

/// A lightweight company reference used inside
/// [`InvolvedCompany`](crate::models::games::InvolvedCompany).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompanyRef {
    /// Unique company identifier.
    pub id: u64,

    /// Company name.
    #[serde(default)]
    pub name: Option<String>,
}

impl FromId for CompanyRef {
    fn from_id(id: u64) -> Self {
        Self { id, name: None }
    }
}

/// A lightweight event reference used inside
/// [`EventLogo`](crate::models::imagery::EventLogo).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventRef {
    /// Unique event identifier.
    pub id: u64,

    /// Event name.
    #[serde(default)]
    pub name: Option<String>,
}

impl FromId for EventRef {
    fn from_id(id: u64) -> Self {
        Self { id, name: None }
    }
}

/// A lightweight game reference used inside
/// [`Game`](crate::models::games::Game)
/// to prevent recursive Game references.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameRef {
    /// Unique game identifier.
    pub id: u64,

    /// Game name.
    #[serde(default)]
    pub name: Option<String>,
}

impl FromId for GameRef {
    fn from_id(id: u64) -> Self {
        Self { id, name: None }
    }
}

/// A lightweight game localization reference used inside
/// [`Cover`](crate::models::imagery::Cover)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameLocalizationRef {
    /// Unique game localization identifier.
    pub id: u64,

    /// Game localization name.
    #[serde(default)]
    pub name: Option<String>,
}

impl FromId for GameLocalizationRef {
    fn from_id(id: u64) -> Self {
        Self { id, name: None }
    }
}

/// A lightweight platform reference used inside
/// [`Game`](crate::models::games::Game).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlatformRef {
    /// Unique platform identifier.
    pub id: u64,

    /// Platform display name.
    #[serde(default)]
    pub name: Option<String>,

    /// Short abbreviation (e.g. `"PS4"`, `"XB1"`).
    #[serde(default)]
    pub abbreviation: Option<String>,
}

impl FromId for PlatformRef {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            name: None,
            abbreviation: None,
        }
    }
}

/// A lightweight theme reference used inside
/// [`Game`](crate::models::games::Game).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeRef {
    /// Unique theme identifier.
    pub id: u64,

    /// Theme name.
    #[serde(default)]
    pub name: Option<String>,
}

impl FromId for ThemeRef {
    fn from_id(id: u64) -> Self {
        Self { id, name: None }
    }
}
