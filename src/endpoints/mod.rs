//! # Endpoints
//!
//! Endpoint-specific query interfaces for the IGDB API.
//!
//! ## Trait Hierarchy
//!
//! ```text
//! ┌─────────────────────┐
//! │       Endpoint      │  <- select, where, sort, limit, offset, expand
//! └──────────┬──────────┘
//!            │
//!    ┌───────┴────────┐
//!    v                v
//! ┌──────────┐  ┌───────────────┐
//! │Searchable│  │NameFilterable │
//! │ .search()│  │.find_by_name()│
//! └──────────┘  └───────────────┘
//! ```
//!
//! ## Design Pattern
//!
//! ```text
//! client.endpoint()           // Returns endpoint handle
//!     .select(&[...])         // Field selection
//!     .where_clause("...")    // Filtering
//!     .sort_by("field", true) // Sorting
//!     .limit(N)               // Pagination
//!     .offset(M)              // Pagination
//!     .expand("rel", &[...])  // Nested expansion
//!     .execute()              // Execute and deserialize
//!     .await
//! ```

/// Generates a typed endpoint struct with full trait implementations.
///
/// # Variants
///
/// ```rust,ignore
/// define_endpoint!(/// docs  pub struct Foos => "foos", Foo);
/// define_endpoint!(/// docs  pub struct Bars => "bars", Bar, searchable);
/// define_endpoint!(/// docs  pub struct Bazs => "bazs", Baz, name_filterable);
/// ```
macro_rules! define_endpoint {
    (@base
        $(#[$meta:meta])*
        $vis:vis struct $name:ident => $path:literal, $model:ty
    ) => {
        $(#[$meta])*
        $vis struct $name {
            client: crate::client::IGDBClient,
            builder: crate::query::QueryBuilder,
        }

        impl crate::endpoints::traits::Endpoint for $name {
            fn builder_mut(&mut self) -> &mut crate::query::QueryBuilder {
                &mut self.builder
            }
            fn builder_ref(&self) -> &crate::query::QueryBuilder {
                &self.builder
            }
        }

        impl $name {
            /// Creates a new endpoint handler.
            pub fn new(client: crate::client::IGDBClient) -> Self {
                Self {
                    client,
                    builder: crate::query::QueryBuilder::new(),
                }
            }

            /// Executes the built query and returns all matching results.
            pub async fn execute(self) -> crate::error::Result<Vec<$model>> {
                let query = self.builder.build();
                self.client.execute_query($path, &query).await
            }

            /// Executes the built query with `limit 1` and returns the
            /// first result, or `None`.
            pub async fn execute_one(mut self) -> crate::error::Result<Option<$model>> {
                self.builder = self.builder.limit(1);
                let query = self.builder.build();
                let results: Vec<$model> =
                    self.client.execute_query($path, &query).await?;
                Ok(results.into_iter().next())
            }
        }
    };

    // No extra trait
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident => $path:literal, $model:ty
    ) => {
        define_endpoint!(@base $(#[$meta])* $vis struct $name => $path, $model);
    };

    // Searchable
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident => $path:literal, $model:ty, searchable
    ) => {
        define_endpoint!(@base $(#[$meta])* $vis struct $name => $path, $model);
        impl crate::endpoints::traits::Searchable for $name {}
    };

    // NameFilterable
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident => $path:literal, $model:ty, name_filterable
    ) => {
        define_endpoint!(@base $(#[$meta])* $vis struct $name => $path, $model);
        impl crate::endpoints::traits::NameFilterable for $name {}
    };
}

pub mod traits;
