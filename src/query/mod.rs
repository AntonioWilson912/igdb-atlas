//! # Query Builder
//!
//! Provides a fluent builder for constructing IGDB Apicalypse queries.
//!
//! ## Apicalypse Syntax Reference
//!
//! IGDB uses a custom query language called Apicalypse:
//!
//! - **Field selection**: `fields name, rating, platforms;`
//! - **Search**: `search "The Witcher";`
//! - **Where clauses**: `where rating > 80 & platforms = 48;`
//! - **Sort**: `sort rating desc;`
//! - **Limit**: `limit 10;`
//! - **Offset**: `offset 20;`
//! - **Expansions**: `fields platforms.*, developer_company.{name, country};`
//!
//! ## Builder Usage
//!
//! ```rust
//! use igdb_atlas::QueryBuilder;
//!
//! let query = QueryBuilder::new()
//!     .select(&["name", "rating", "platforms"])
//!     .search("Zelda")
//!     .where_clause("rating > 80")
//!     .sort_by("rating", true)  // true = descending
//!     .limit(10)
//!     .offset(5)
//!     .expand("platforms", &["name", "logo"])
//!     .build();
//!
//! assert!(query.contains("fields name,rating,platforms"));
//! assert!(query.contains("search \"Zelda\""));
//! assert!(query.contains("where rating > 80"));
//! assert!(query.contains("sort rating desc"));
//! assert!(query.contains("limit 10"));
//! assert!(query.contains("offset 5"));
//! ```

pub mod builder;

pub use builder::QueryBuilder;
