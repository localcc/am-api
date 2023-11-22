//! Request builder that supports views

use crate::request::context::RequestContext;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// A trait for getting information about a view type
pub trait ViewTrait: Display {
    /// Get view object name
    fn get_object(&self) -> &'static str;
}

/// View storage
#[derive(Default)]
pub struct ViewStorage {
    storage: HashMap<&'static str, HashSet<String>>,
}

impl ViewStorage {
    /// Add a view to this storage
    pub fn add_view(&mut self, view: impl ViewTrait) {
        self.storage
            .entry(view.get_object())
            .or_default()
            .insert(view.to_string());
    }

    /// Build view query parameters draining this storage
    pub fn build_query_drain(&mut self, request_context: &mut RequestContext) {
        for (object, extend) in self.storage.drain() {
            request_context.query.push((
                format!("views[{}]", object),
                extend.into_iter().collect::<Vec<_>>().join(","),
            ));
        }
    }
}
