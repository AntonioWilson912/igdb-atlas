//! # Company Model
//!
//! Represents a game company from the IGDB v4 `/companies` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::companies::Company;
//!
//! let json = r#"{
//!     "id": 227,
//!     "name": "CD Projekt",
//!     "country": 36,
//!     "description": "Polish game developer known for The Witcher series.",
//!     "slug": "cd-projekt",
//!     "start_date": 1104537600
//! }"#;
//!
//! let company: Company = serde_json::from_str(json).unwrap();
//! assert_eq!(company.name, Some("CD Projekt".to_string()));
//! assert_eq!(company.country, Some(36));
//! ```

use serde::{Deserialize, Serialize};

use crate::{
    DateFormat, Game,
    models::{
        common::{CompanyRef, GameRef},
        companies::CompanyStatus,
        id_or_object::{FromId, deserialize_id_or_object, deserialize_id_or_object_vec},
        imagery::CompanyLogo,
        timestamp::{format_timestamp, format_timestamp_pretty},
        websites::CompanyWebsite,
    },
};

/// A game development or publishing company.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    /// Unique company identifier.
    pub id: u64,

    /// Unix timestamp for when this company acquired a new ID
    /// (mergers / restructuring).
    #[serde(default)]
    pub change_date: Option<i64>,

    /// The format of the change date.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub change_date_format: Option<DateFormat>,

    /// The company that replaced this one after a merger / restructure.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub changed_company_id: Option<CompanyRef>,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// ISO 3166-1 numeric country code.
    #[serde(default)]
    pub country: Option<u16>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// Free-text description of the company.
    #[serde(default)]
    pub description: Option<String>,

    /// An array of games that a company has developed.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub developed: Option<Vec<GameRef>>,

    /// The company's logo.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub logo: Option<CompanyLogo>,

    /// The company's name.
    #[serde(default)]
    pub name: Option<String>,

    /// A company with a controlling interest in a specific company.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub parent: Option<CompanyRef>,

    /// An array of games that a company has published.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub published: Option<Vec<Game>>,

    /// URL-safe, unique, lower-case version of the name.
    #[serde(default)]
    pub slug: Option<String>,

    /// Unix timestamp of when the company was founded.
    #[serde(default)]
    pub start_date: Option<i64>,

    /// The format of the start date.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub start_date_format: Option<DateFormat>,

    /// The status of the company.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub status: Option<CompanyStatus>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// The IGDB page URL.
    #[serde(default)]
    pub url: Option<String>,

    /// The company's official websites.
    #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
    pub websites: Option<Vec<CompanyWebsite>>,
}

impl Company {
    /// Returns the company name or `"Unknown Company"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Company")
    }

    /// Returns `true` if this company has developed any games.
    pub fn is_developer(&self) -> bool {
        self.developed
            .as_ref()
            .map(|d| !d.is_empty())
            .unwrap_or(false)
    }

    /// Returns `true` if this company has published any games.
    pub fn is_publisher(&self) -> bool {
        self.published
            .as_ref()
            .map(|p| !p.is_empty())
            .unwrap_or(false)
    }
}

impl Default for Company {
    fn default() -> Self {
        Self {
            id: 0,
            change_date: None,
            change_date_format: None,
            changed_company_id: None,
            checksum: None,
            country: None,
            created_at: None,
            description: None,
            developed: None,
            logo: None,
            name: None,
            parent: None,
            published: None,
            slug: None,
            start_date: None,
            start_date_format: None,
            status: None,
            updated_at: None,
            url: None,
            websites: None,
        }
    }
}

impl FromId for Company {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Company [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
        }
        if let Some(ref slug) = self.slug {
            writeln!(f, "  Slug: {}", slug)?;
        }
        if let Some(ref desc) = self.description {
            let truncated = if desc.len() > 200 {
                format!("{}...", &desc[..200])
            } else {
                desc.clone()
            };
            writeln!(f, "  Description: {}", truncated)?;
        }
        if let Some(country) = self.country {
            writeln!(f, "  Country Code: {}", country)?;
        }
        if let Some(ts) = self.start_date {
            if let Some(date) = format_timestamp_pretty(Some(ts)) {
                writeln!(f, "  Founded: {}", date)?;
            }
        }
        if let Some(ref url) = self.url {
            writeln!(f, "  URL: {}", url)?;
        }
        if let Some(ref developed) = self.developed {
            if !developed.is_empty() {
                writeln!(f, "  Games Developed: {}", developed.len())?;
            }
        }
        if let Some(ref published) = self.published {
            if !published.is_empty() {
                writeln!(f, "  Games Published: {}", published.len())?;
            }
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
