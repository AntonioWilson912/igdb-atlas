//! # Collections Endpoint
//!
//! Fluent interfaces for IGDB collection-related endpoints.
//!
//! | Struct | API Path | Model |
//! |--------|----------|-------|
//! | [`CollectionsEndpoint`] | `/collections` | [`Collection`] |
//! | [`CollectionMembershipsEndpoint`] | `/collection_memberships` | [`CollectionMembership`] |
//! | [`CollectionMembershipTypesEndpoint`] | `/collection_membership_types` | [`CollectionMembershipType`] |
//! | [`CollectionRelationsEndpoint`] | `/collection_relations` | [`CollectionRelation`] |
//! | [`CollectionRelationTypesEndpoint`] | `/collection_relation_types` | [`CollectionRelationType`] |
//! | [`CollectionTypesEndpoint`] | `/collection_types` | [`CollectionType`] |

use crate::models::collections::{
    Collection, CollectionMembership, CollectionMembershipType, CollectionRelation,
    CollectionRelationType, CollectionType,
};

define_endpoint! {
    /// `/collections` - searchable.
    pub struct CollectionsEndpoint => "collections", Collection, searchable
}

define_endpoint! {
    /// `/collection_memberships` - not searchable.
    pub struct CollectionMembershipsEndpoint => "collection_memberships", CollectionMembership
}

define_endpoint! {
    /// `/collection_membership_types` - not searchable.
    pub struct CollectionMembershipTypesEndpoint => "collection_membership_types", CollectionMembershipType
}

define_endpoint! {
    /// `/collection_relations` - not searchable.
    pub struct CollectionRelationsEndpoint => "collection_relations", CollectionRelation
}

define_endpoint! {
    /// `/collection_relation_types` - not searchable.
    pub struct CollectionRelationTypesEndpoint => "collection_relation_types", CollectionRelationType
}

define_endpoint! {
    /// `/collection_types` - not searchable.
    pub struct CollectionTypesEndpoint => "collection_types", CollectionType, name_filterable
}
