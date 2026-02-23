//! # Company Type History Model
//!
//! Represents a company type history record from the IGDB v4
//! `/company_type_histories` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::companies::CompanyTypeHistory;
//!
//! let json = r#"{
//!     "id": 1000,
//!     "company": 5953,
//! 	"company_type": 2,
//! 	"parent_company": 70
//! }"#;
//!
//! let company_type: CompanyTypeHistory = serde_json::from_str(json).unwrap();
//! assert_eq!(company_type.company.as_ref().map(|o| o.id), Some(5953));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{
    companies::{Company, CompanyType},
    id_or_object::{FromId, deserialize_id_or_object},
    timestamp::format_timestamp,
};

/// A company type record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyTypeHistory {
    /// Unique company type identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// The company.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub company: Option<Company>,

    /// The company type.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub company_type: Option<CompanyType>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The parent company.
    #[serde(default, deserialize_with = "deserialize_id_or_object")]
    pub parent_company: Option<Company>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl Default for CompanyTypeHistory {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            company: None,
            company_type: None,
            created_at: None,
            parent_company: None,
            updated_at: None,
        }
    }
}

impl FromId for CompanyTypeHistory {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CompanyTypeHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CompanyTypeHistory [{}]", self.id)?;
        if let Some(ref company) = self.company {
            writeln!(f, "  Company: {}", company)?;
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
