//! # Involved Company Model
//!
//! Represents a company's involvement with a game from the IGDB v4
//! `/involved_companies` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::games::InvolvedCompany;
//!
//! let json = r#"{
//!     "id": 1,
//!     "company": 227,
//!     "game": 1942,
//!     "developer": true,
//!     "publisher": false,
//!     "porting": false,
//!     "supporting": false
//! }"#;
//!
//! let ic: InvolvedCompany = serde_json::from_str(json).unwrap();
//! assert_eq!(ic.developer, Some(true));
//! assert!(ic.is_developer());
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    common::CompanyRef,
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A company's involvement with a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvolvedCompany {
    /// Unique involved company identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Reference to the company. Bare ID when unexpanded, full
    /// [`CompanyRef`] object when expanded.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub company: Option<CompanyRef>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Whether the company was a developer.
    #[serde(default)]
    pub developer: Option<bool>,

    /// Reference ID to the game.
    #[serde(default)]
    pub game: Option<u64>,

    /// Whether the company ported the game to another platform.
    #[serde(default)]
    pub porting: Option<bool>,

    /// Whether the company was a publisher.
    #[serde(default)]
    pub publisher: Option<bool>,

    /// Whether the company had a supporting role.
    #[serde(default, alias = "supporter")]
    pub supporting: Option<bool>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl InvolvedCompany {
    /// Returns the company name if the reference is expanded, or `None`.
    pub fn company_name(&self) -> Option<&str> {
        self.company.as_ref().and_then(|c| c.name.as_deref())
    }

    /// Returns `true` if this company was a developer.
    pub fn is_developer(&self) -> bool {
        self.developer == Some(true)
    }

    /// Returns `true` if this company was a publisher.
    pub fn is_publisher(&self) -> bool {
        self.publisher == Some(true)
    }

    /// Returns `true` if this company ported the game.
    pub fn is_port(&self) -> bool {
        self.porting == Some(true)
    }

    /// Returns `true` if this company had a supporting role.
    pub fn is_supporting(&self) -> bool {
        self.supporting == Some(true)
    }

    /// Returns a list of role labels for this company.
    pub fn roles(&self) -> Vec<&'static str> {
        let mut roles = Vec::new();
        if self.is_developer() {
            roles.push("Developer");
        }
        if self.is_publisher() {
            roles.push("Publisher");
        }
        if self.is_port() {
            roles.push("Port");
        }
        if self.is_supporting() {
            roles.push("Support");
        }
        roles
    }
}

impl Default for InvolvedCompany {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            company: None,
            created_at: None,
            developer: None,
            game: None,
            porting: None,
            publisher: None,
            supporting: None,
            updated_at: None,
        }
    }
}

impl FromId for InvolvedCompany {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for InvolvedCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvolvedCompany [{}]", self.id)?;
        if let Some(name) = self.company_name() {
            write!(f, " {}", name)?;
        } else if let Some(ref c) = self.company {
            write!(f, " company={}", c.id)?;
        }
        let roles = self.roles();
        if !roles.is_empty() {
            write!(f, " ({})", roles.join(", "))?;
        }
        if let Some(ts) = self.updated_at {
            if let Some(d) = format_timestamp(Some(ts)) {
                write!(f, " updated {}", d)?;
            }
        }
        writeln!(f)
    }
}
