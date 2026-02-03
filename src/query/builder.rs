//! # Query Builder
//!
//! Constructs Apicalypse query strings for the IGDB API.
//!
//! ## Apicalypse Syntax Reference
//!
//! The IGDB API uses a custom query language called Apicalypse:
//!
//! ```text
//! fields name, rating, platforms.name, platforms.abbreviation;
//! search "Zelda";
//! where rating > 80 & platforms = 48;
//! sort rating desc;
//! limit 10;
//! offset 0;
//! ```
//!
//! ### Field Expansion
//!
//! Nested objects are expanded by using dot notation in the `fields` clause:
//!
//! - `platforms.*` — all sub-fields of platforms
//! - `platforms.name,platforms.abbreviation` — specific sub-fields
//! - `involved_companies.company.name` — deep nesting
//!
//! Without expansion, nested references return as bare integer IDs.
//!
//! ## Example
//!
//! ```rust
//! use igdb_atlas::QueryBuilder;
//!
//! let query = QueryBuilder::new()
//!     .select(&["name", "rating"])
//!     .search("Zelda")
//!     .where_clause("rating > 80")
//!     .sort_by("rating", true)
//!     .limit(10)
//!     .expand("platforms", &["name", "abbreviation"])
//!     .build();
//!
//! assert!(query.contains("platforms.name"));
//! assert!(query.contains("platforms.abbreviation"));
//! ```

/// Builder for constructing Apicalypse query strings.
///
/// # Examples
///
/// ```rust
/// use igdb_atlas::QueryBuilder;
///
/// let query = QueryBuilder::new()
///     .select(&["name", "rating"])
///     .search("Dark Souls")
///     .limit(5)
///     .build();
///
/// assert!(query.contains("fields name,rating;"));
/// assert!(query.contains("search \"Dark Souls\";"));
/// assert!(query.contains("limit 5;"));
/// ```
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    fields: Vec<String>,
    search_term: Option<String>,
    where_clauses: Vec<String>,
    sort_field: Option<String>,
    sort_desc: bool,
    limit_val: Option<u32>,
    offset_val: Option<u32>,
    /// Each expansion is (parent_field, vec_of_sub_fields).
    /// Empty sub_fields means expand all (`parent.*`).
    expansions: Vec<(String, Vec<String>)>,
}

impl QueryBuilder {
    /// Creates a new empty query builder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let builder = QueryBuilder::new();
    /// assert_eq!(builder.field_count(), 0);
    /// assert!(!builder.has_search());
    /// ```
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            search_term: None,
            where_clauses: Vec::new(),
            sort_field: None,
            sort_desc: false,
            limit_val: None,
            offset_val: None,
            expansions: Vec::new(),
        }
    }

    /// Sets the fields to return.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new().select(&["name", "rating"]).build();
    /// assert!(q.contains("fields name,rating;"));
    /// ```
    pub fn select(mut self, fields: &[&str]) -> Self {
        self.fields = fields.iter().map(|f| f.to_string()).collect();
        self
    }

    /// Appends a single field to the selection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new()
    ///     .select(&["name"])
    ///     .add_field("rating")
    ///     .build();
    /// assert!(q.contains("name"));
    /// assert!(q.contains("rating"));
    /// ```
    pub fn add_field(mut self, field: &str) -> Self {
        self.fields.push(field.to_string());
        self
    }

    /// Sets the search term.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new().search("Zelda").build();
    /// assert!(q.contains("search \"Zelda\";"));
    /// ```
    pub fn search(mut self, term: &str) -> Self {
        self.search_term = Some(term.to_string());
        self
    }

    /// Sets or replaces the WHERE clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new().where_clause("rating > 80").build();
    /// assert!(q.contains("where rating > 80;"));
    /// ```
    pub fn where_clause(mut self, clause: &str) -> Self {
        self.where_clauses = vec![clause.to_string()];
        self
    }

    /// Appends an AND condition to the WHERE clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new()
    ///     .where_clause("rating > 80")
    ///     .and_where("platforms = 48")
    ///     .build();
    /// assert!(q.contains("where rating > 80 & platforms = 48;"));
    /// ```
    pub fn and_where(mut self, clause: &str) -> Self {
        self.where_clauses.push(clause.to_string());
        self
    }

    /// Sets the sort field and direction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new().sort_by("rating", true).build();
    /// assert!(q.contains("sort rating desc;"));
    /// ```
    pub fn sort_by(mut self, field: &str, descending: bool) -> Self {
        self.sort_field = Some(field.to_string());
        self.sort_desc = descending;
        self
    }

    /// Sets the result limit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new().limit(25).build();
    /// assert!(q.contains("limit 25;"));
    /// ```
    pub fn limit(mut self, n: u32) -> Self {
        self.limit_val = Some(n);
        self
    }

    /// Sets the pagination offset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new().offset(50).build();
    /// assert!(q.contains("offset 50;"));
    /// ```
    pub fn offset(mut self, n: u32) -> Self {
        self.offset_val = Some(n);
        self
    }

    /// Adds a field expansion using Apicalypse dot notation.
    ///
    /// When `sub_fields` is empty, expands all sub-fields (`parent.*`).
    /// When `sub_fields` has entries, expands each one individually
    /// (`parent.field1, parent.field2`).
    ///
    /// The expanded fields are automatically added to the `fields` clause
    /// in the final query. You do **not** need to include the parent field
    /// in your `select()` call — it will be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// // Expand all platform sub-fields
    /// let q = QueryBuilder::new()
    ///     .select(&["name"])
    ///     .expand("platforms", &[])
    ///     .build();
    /// assert!(q.contains("platforms.*"));
    ///
    /// // Expand specific sub-fields
    /// let q = QueryBuilder::new()
    ///     .select(&["name"])
    ///     .expand("platforms", &["name", "abbreviation"])
    ///     .build();
    /// assert!(q.contains("platforms.name"));
    /// assert!(q.contains("platforms.abbreviation"));
    /// ```
    pub fn expand(mut self, parent: &str, sub_fields: &[&str]) -> Self {
        self.expansions.push((
            parent.to_string(),
            sub_fields.iter().map(|f| f.to_string()).collect(),
        ));
        self
    }

    /// Builds the final Apicalypse query string.
    ///
    /// # Field Resolution
    ///
    /// The `fields` clause is built by combining:
    /// 1. Explicitly selected fields (from `select()` / `add_field()`)
    /// 2. Expanded field references (from `expand()`)
    ///
    /// If no fields are selected and no expansions exist, defaults to `fields *;`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let q = QueryBuilder::new()
    ///     .select(&["name", "rating"])
    ///     .search("Zelda")
    ///     .where_clause("rating > 80")
    ///     .sort_by("rating", true)
    ///     .limit(10)
    ///     .expand("platforms", &["name", "abbreviation"])
    ///     .build();
    ///
    /// // All parts present
    /// assert!(q.contains("fields name,rating,platforms.name,platforms.abbreviation;"));
    /// assert!(q.contains("search \"Zelda\";"));
    /// assert!(q.contains("where rating > 80;"));
    /// assert!(q.contains("sort rating desc;"));
    /// assert!(q.contains("limit 10;"));
    /// ```
    pub fn build(&self) -> String {
        let mut parts = Vec::new();

        // Build fields clause with expansions
        let mut all_fields: Vec<String> = Vec::new();

        // Add explicitly selected fields (skip "*" if we have expansions)
        for f in &self.fields {
            if f == "*" && !self.expansions.is_empty() {
                // When using * with expansions, we still need * for
                // the top-level fields
                all_fields.push("*".to_string());
            } else {
                all_fields.push(f.clone());
            }
        }

        // Add expansion fields using dot notation
        for (parent, sub_fields) in &self.expansions {
            // Remove the parent from top-level fields if present
            // (it will be replaced by the dot-notation version)
            all_fields.retain(|f| f != parent);

            if sub_fields.is_empty() {
                all_fields.push(format!("{}.*", parent));
            } else {
                for sf in sub_fields {
                    all_fields.push(format!("{}.{}", parent, sf));
                }
            }
        }

        if all_fields.is_empty() {
            parts.push("fields *;".to_string());
        } else {
            parts.push(format!("fields {};", all_fields.join(",")));
        }

        // Search
        if let Some(ref term) = self.search_term {
            parts.push(format!("search \"{}\";", term));
        }

        // Where
        if !self.where_clauses.is_empty() {
            parts.push(format!("where {};", self.where_clauses.join(" & ")));
        }

        // Sort
        if let Some(ref field) = self.sort_field {
            let dir = if self.sort_desc { "desc" } else { "asc" };
            parts.push(format!("sort {} {};", field, dir));
        }

        // Limit
        if let Some(n) = self.limit_val {
            parts.push(format!("limit {};", n));
        }

        // Offset
        if let Some(n) = self.offset_val {
            parts.push(format!("offset {};", n));
        }

        parts.join(" ")
    }

    /// Returns the number of explicitly selected fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let b = QueryBuilder::new().select(&["name", "rating"]);
    /// assert_eq!(b.field_count(), 2);
    /// ```
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Returns `true` if a search term has been set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// assert!(!QueryBuilder::new().has_search());
    /// assert!(QueryBuilder::new().search("test").has_search());
    /// ```
    pub fn has_search(&self) -> bool {
        self.search_term.is_some()
    }

    /// Returns the number of WHERE clauses.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let b = QueryBuilder::new()
    ///     .where_clause("a = 1")
    ///     .and_where("b = 2");
    /// assert_eq!(b.where_count(), 2);
    /// ```
    pub fn where_count(&self) -> usize {
        self.where_clauses.len()
    }

    /// Returns the number of field expansions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use igdb_atlas::QueryBuilder;
    ///
    /// let b = QueryBuilder::new()
    ///     .expand("platforms", &["name"])
    ///     .expand("genres", &["name"]);
    /// assert_eq!(b.expansion_count(), 2);
    /// ```
    pub fn expansion_count(&self) -> usize {
        self.expansions.len()
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
