//! # Endpoint Traits
//!
//! Defines the trait hierarchy for IGDB endpoint builders.
//!
//! ## Traits
//!
//! - [`Endpoint`] - Base trait for all endpoints (field selection, filtering,
//!   sorting, pagination, expansion)
//! - [`Searchable`] - For endpoints that support IGDB's built-in search
//!   (Games, Platforms, Characters, Collections, Themes)
//! - [`NameFilterable`] - For endpoints without search support, provides
//!   a `find_by_name` convenience method using a `where` clause
//!
//! ## IGDB Search Support
//!
//! Per the IGDB documentation, the `search` keyword is only available on:
//! Characters, Collections, Games, Platforms, and Themes.
//!
//! All other endpoints must use `where` clauses for name-based lookups.

use crate::query::QueryBuilder;

/// Base trait implemented by every endpoint builder.
///
/// Provides the common query-building methods that all IGDB endpoints
/// share: field selection, filtering, sorting, pagination, and expansion.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::endpoints::traits::Endpoint;
/// use igdb_atlas::query::QueryBuilder;
///
/// // Any endpoint implementing this trait supports these methods
/// fn build_common_query<E: Endpoint>(endpoint: E) -> E {
///     endpoint
///         .select(&["name", "id"])
///         .where_clause("id > 100")
///         .sort_by("name", false)
///         .limit(10)
///         .offset(0)
/// }
/// ```
pub trait Endpoint: Sized {
    /// Returns a mutable reference to the inner query builder.
    fn builder_mut(&mut self) -> &mut QueryBuilder;

    /// Returns a reference to the inner query builder.
    fn builder_ref(&self) -> &QueryBuilder;

    /// Sets the fields to select in the response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::{IGDBClient, ClientConfig};
    /// use igdb_atlas::endpoints::traits::Endpoint;
    ///
    /// async fn example() {
    ///     let config = ClientConfig::new("id", "secret");
    ///     let client = IGDBClient::new(config).await.unwrap();
    ///     let _ = client.games().select(&["name", "rating"]);
    /// }
    /// ```
    fn select(mut self, fields: &[&str]) -> Self {
        let b = self.builder_mut().clone().select(fields);
        *self.builder_mut() = b;
        self
    }

    /// Adds a WHERE clause for filtering results.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::{IGDBClient, ClientConfig};
    /// use igdb_atlas::endpoints::traits::Endpoint;
    ///
    /// async fn example() {
    ///     let config = ClientConfig::new("id", "secret");
    ///     let client = IGDBClient::new(config).await.unwrap();
    ///     let _ = client.games().where_clause("rating > 80");
    /// }
    /// ```
    fn where_clause(mut self, clause: &str) -> Self {
        let b = self.builder_mut().clone().where_clause(clause);
        *self.builder_mut() = b;
        self
    }

    /// Adds an additional AND condition to the WHERE clause.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use igdb_atlas::{IGDBClient, ClientConfig};
    /// use igdb_atlas::endpoints::traits::Endpoint;
    ///
    /// async fn example() {
    ///     let config = ClientConfig::new("id", "secret");
    ///     let client = IGDBClient::new(config).await.unwrap();
    ///     let _ = client.games()
    ///         .where_clause("rating > 80")
    ///         .and_where("platforms = 48");
    /// }
    /// ```
    fn and_where(mut self, clause: &str) -> Self {
        let b = self.builder_mut().clone().and_where(clause);
        *self.builder_mut() = b;
        self
    }

    /// Sets the sort field and direction.
    ///
    /// # Parameters
    ///
    /// - `field`: The field name to sort by
    /// - `descending`: `true` for descending, `false` for ascending
    fn sort_by(mut self, field: &str, descending: bool) -> Self {
        let b = self.builder_mut().clone().sort_by(field, descending);
        *self.builder_mut() = b;
        self
    }

    /// Sets the maximum number of results to return.
    ///
    /// IGDB allows up to 500 results per request.
    fn limit(mut self, n: u32) -> Self {
        let b = self.builder_mut().clone().limit(n);
        *self.builder_mut() = b;
        self
    }

    /// Sets the pagination offset.
    fn offset(mut self, n: u32) -> Self {
        let b = self.builder_mut().clone().offset(n);
        *self.builder_mut() = b;
        self
    }

    /// Adds a nested field expansion.
    ///
    /// Pass empty `sub_fields` to expand all sub-fields.
    fn expand(mut self, parent: &str, sub_fields: &[&str]) -> Self {
        let b = self.builder_mut().clone().expand(parent, sub_fields);
        *self.builder_mut() = b;
        self
    }

    /// Returns the raw query string that would be sent.
    ///
    /// Useful for debugging or logging.
    fn build_query(&self) -> String {
        self.builder_ref().build()
    }
}

/// Trait for endpoints that support IGDB's built-in `search` keyword.
///
/// Per the IGDB documentation, search is available on:
/// Characters, Collections, Games, Platforms, and Themes.
///
/// # Examples
///
/// ```rust,no_run
/// use igdb_atlas::{IGDBClient, ClientConfig};
/// use igdb_atlas::endpoints::traits::{Endpoint, Searchable};
///
/// async fn example() {
///     let config = ClientConfig::new("id", "secret");
///     let client = IGDBClient::new(config).await.unwrap();
///
///     let games = client.games()
///         .search("Zelda")
///         .select(&["name", "rating"])
///         .limit(5)
///         .execute()
///         .await
///         .unwrap();
/// }
/// ```
pub trait Searchable: Endpoint {
    /// Sets the search term using IGDB's built-in search.
    ///
    /// This performs a full-text search on the endpoint's primary
    /// text fields (typically `name`). Results are ranked by relevance.
    fn search(mut self, term: &str) -> Self {
        let b = self.builder_mut().clone().search(term);
        *self.builder_mut() = b;
        self
    }
}

/// Trait for endpoints that do **not** support IGDB's built-in search.
///
/// Provides [`find_by_name`](NameFilterable::find_by_name) as a
/// convenience method that generates a case-insensitive `where` clause.
///
/// The generated query uses IGDB's `~` (case-insensitive) operator
/// with wildcard matching: `where name ~ *"term"*;`
///
/// # Examples
///
/// ```rust,no_run
/// use igdb_atlas::{IGDBClient, ClientConfig};
/// use igdb_atlas::endpoints::traits::{Endpoint, NameFilterable};
///
/// async fn example() {
///     let config = ClientConfig::new("id", "secret");
///     let client = IGDBClient::new(config).await.unwrap();
///
///     let franchises = client.franchises()
///         .find_by_name("The Blues Brothers")
///         .select(&["name", "slug"])
///         .limit(5)
///         .execute()
///         .await
///         .unwrap();
/// }
/// ```
pub trait NameFilterable: Endpoint {
    /// Filters results by name using a case-insensitive contains match.
    ///
    /// Generates: `where name ~ *"term"*;`
    fn find_by_name(mut self, name: &str) -> Self {
        let clause = format!("name ~ *\"{}\"*", name);
        let b = self.builder_mut().clone().where_clause(&clause);
        *self.builder_mut() = b;
        self
    }
}
