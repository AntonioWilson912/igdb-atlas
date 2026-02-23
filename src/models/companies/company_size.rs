//! # Company Size Model
//!
//! Represents a company size from the IGDB v4
//! `/company_sizes` endpoint.
//!
//! # Examples
//!
//! ```rust
//! use serde_json;
//! use igdb_atlas::models::companies::CompanySize;
//!
//! let json = r#"{
//!     "id": 2,
//!     "name": "11-50 employees"
//! }"#;
//!
//! let company_size: CompanySize = serde_json::from_str(json).unwrap();
//! assert_eq!(company_size.name, Some("11-50 employees".to_string()));
//! ```

use serde::{Deserialize, Serialize};

use crate::models::{id_or_object::FromId, timestamp::format_timestamp};

/// A company size record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanySize {
    /// Unique company size identifier.
    pub id: u64,

    /// SHA-1 checksum / hash of the object.
    #[serde(default)]
    pub checksum: Option<String>,

    /// Unix timestamp when this entry was first added to IGDB.
    #[serde(default)]
    pub created_at: Option<i64>,

    /// The name of the company size.
    #[serde(default)]
    pub name: Option<String>,

    /// Unix timestamp of the last update to this entry.
    #[serde(default)]
    pub updated_at: Option<i64>,
}

impl CompanySize {
    /// Returns the company size name or `"Unknown Company Size"`.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown Company Size")
    }
}

impl Default for CompanySize {
    fn default() -> Self {
        Self {
            id: 0,
            checksum: None,
            created_at: None,
            name: None,
            updated_at: None,
        }
    }
}

impl FromId for CompanySize {
    fn from_id(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl std::fmt::Display for CompanySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CompanySize [{}]", self.id)?;
        if let Some(ref name) = self.name {
            writeln!(f, "  Name: {}", name)?;
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
