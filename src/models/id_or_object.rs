//! # ID-or-Object Deserialization
//!
//! The IGDB API returns nested references in two forms depending on
//! whether the field was expanded in the query:
//!
//! - **Unexpanded**: a bare integer ID (e.g., `48`)
//! - **Expanded**: a full JSON object (e.g., `{"id": 48, "name": "PS4"}`)
//!
//! This module provides [`deserialize_id_or_object`] and
//! [`deserialize_id_or_object_vec`] which handle both forms
//! transparently for any struct that implements [`FromId`].
//!
//! ## How It Works
//!
//! When the deserializer encounters an integer, it constructs an
//! instance of the target type using `FromId::from_id(id)`.
//! When it encounters an object, it deserializes normally.
//!
//! ## Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use igdb_atlas::models::id_or_object::{deserialize_id_or_object_vec, FromId};
//!
//! #[derive(Debug, Clone, Default, Serialize, Deserialize)]
//! struct MyRef {
//!     pub id: u64,
//!     #[serde(default)]
//!     pub name: Option<String>,
//! }
//!
//! impl FromId for MyRef {
//!     fn from_id(id: u64) -> Self {
//!         Self { id, name: None }
//!     }
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Parent {
//!     #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
//!     pub refs: Option<Vec<MyRef>>,
//! }
//!
//! // Works with bare IDs
//! let json = r#"{"refs": [1, 2, 3]}"#;
//! let p: Parent = serde_json::from_str(json).unwrap();
//! assert_eq!(p.refs.as_ref().unwrap()[0].id, 1);
//!
//! // Works with objects
//! let json = r#"{"refs": [{"id": 1, "name": "test"}]}"#;
//! let p: Parent = serde_json::from_str(json).unwrap();
//! assert_eq!(p.refs.as_ref().unwrap()[0].name.as_deref(), Some("test"));
//! ```

use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::Deserialize;
use std::fmt;
use std::marker::PhantomData;

/// Trait for types that can be constructed from just an ID.
///
/// All IGDB reference types (PlatformRef, Genre, etc.) should
/// implement this. The blanket implementation covers any type
/// that is `Default` and has a `set_id` method, but we use a
/// simple trait for clarity.
pub trait FromId: Default {
    /// Creates an instance with only the ID populated.
    fn from_id(id: u64) -> Self;
}

/// Deserializes a value that may be either a bare integer ID
/// or a full JSON object.
///
/// Use this for singular (non-Vec) optional fields:
///
/// ```rust,ignore
/// #[serde(default, deserialize_with = "deserialize_id_or_object")]
/// pub cover: Option<CoverImage>,
/// ```
pub fn deserialize_id_or_object<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromId,
{
    struct IdOrObjectVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for IdOrObjectVisitor<T>
    where
        T: Deserialize<'de> + FromId,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer ID or an object")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
            Ok(Some(T::from_id(value)))
        }

        fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
            Ok(Some(T::from_id(value as u64)))
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let obj = T::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(obj))
        }
    }

    deserializer.deserialize_any(IdOrObjectVisitor(PhantomData))
}

/// Deserializes a `Vec` where each element may be either a bare
/// integer ID or a full JSON object.
///
/// This is the most common case - IGDB arrays like `platforms`,
/// `genres`, `game_modes` etc. can contain mixed bare IDs and objects
/// (though in practice they're uniform per response).
///
/// # Usage
///
/// ```rust,ignore
/// #[serde(default, deserialize_with = "deserialize_id_or_object_vec")]
/// pub platforms: Option<Vec<PlatformRef>>,
/// ```
pub fn deserialize_id_or_object_vec<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromId,
{
    struct VecIdOrObjectVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for VecIdOrObjectVisitor<T>
    where
        T: Deserialize<'de> + FromId,
    {
        type Value = Option<Vec<T>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array of integer IDs or objects, or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(0));

            while let Some(element) = seq.next_element::<IdOrObject<T>>()? {
                items.push(element.into_inner());
            }

            Ok(Some(items))
        }
    }

    deserializer.deserialize_any(VecIdOrObjectVisitor(PhantomData))
}

/// Internal helper: deserializes a single element that is either
/// a bare integer or a JSON object.
///
/// Used by [`deserialize_id_or_object_vec`] for each array element.
struct IdOrObject<T>(T);

impl<T> IdOrObject<T> {
    fn into_inner(self) -> T {
        self.0
    }
}

impl<'de, T> Deserialize<'de> for IdOrObject<T>
where
    T: Deserialize<'de> + FromId,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElementVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for ElementVisitor<T>
        where
            T: Deserialize<'de> + FromId,
        {
            type Value = IdOrObject<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer ID or an object")
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(IdOrObject(T::from_id(value)))
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(IdOrObject(T::from_id(value as u64)))
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let obj = T::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(IdOrObject(obj))
            }
        }

        deserializer.deserialize_any(ElementVisitor(PhantomData))
    }
}
