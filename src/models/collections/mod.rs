//! # Collection Models
//!
//! Models for game collections (series).
//!
//! //! | Module | Endpoint | Description |
//! |--------|----------|-------------|
//! | [`collection`] | `/collections` | The primary [`Collection`] model |
//! | [`collection_membership`] | `/collection_memberships` | |
//! | [`collection_membership_type`] | `/collection_membership_types` | Enum for collection membership types |
//! | [`collection_relation`] | `/collection_relations` | Relationship between collections |
//! | [`collection_relation_type`] | `/collection_relation_types` | Types of relationships between collections |
//! | [`collection_type`] | `/collection_type` | Enum for collection types |

pub mod collection;
pub mod collection_membership;
pub mod collection_membership_type;
pub mod collection_relation;
pub mod collection_relation_type;
pub mod collection_type;

pub use collection::Collection;
pub use collection_membership::CollectionMembership;
pub use collection_membership_type::CollectionMembershipType;
pub use collection_relation::CollectionRelation;
pub use collection_relation_type::CollectionRelationType;
pub use collection_type::CollectionType;
