//! Request builder that supports relations

use crate::request::context::RequestContext;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/// A trait for getting information about a relationship type
pub trait RelationshipTrait: Display {
    /// Get relation object name
    fn get_object(&self) -> &'static str;
}

/// Relationship key
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct RelationshipKey {
    object: &'static str,
    lazy: bool,
}

/// Relationship storage type
#[derive(Default)]
pub struct RelationshipStorage {
    storage: HashMap<RelationshipKey, HashSet<String>>,
}

impl RelationshipStorage {
    /// Add a relationship to this storage
    pub fn add_relationship(&mut self, relationship: impl RelationshipTrait) {
        self.storage
            .entry(RelationshipKey {
                object: relationship.get_object(),
                lazy: false,
            })
            .or_default()
            .insert(relationship.to_string());
    }

    /// Add a lazy relationship to this storage
    pub fn add_relationship_lazy(&mut self, relationship: impl RelationshipTrait) {
        self.storage
            .entry(RelationshipKey {
                object: relationship.get_object(),
                lazy: true,
            })
            .or_default()
            .insert(relationship.to_string());
    }

    /// Build relationship query parameters draining this storage
    pub fn build_query_drain(&mut self, request_context: &mut RequestContext) {
        for (relationship, include) in self.storage.drain() {
            let key = match relationship.lazy {
                true => format!("relate[{}]", relationship.object),
                false => format!("include[{}]", relationship.object),
            };

            let include = include.into_iter().collect::<Vec<_>>().join(",");

            request_context.query.push((key, include));
        }
    }
}
