//! Request builder that supports extensions

use crate::request::context::RequestContext;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// A trait for getting information about an extension type
pub trait ExtensionTrait: Display {
    /// Get extension object name
    fn get_object(&self) -> &'static str;
}

/// Extension storage
#[derive(Default)]
pub struct ExtensionStorage {
    storage: HashMap<&'static str, HashSet<String>>,
}

impl ExtensionStorage {
    /// Add an extension to this storage
    pub fn add_extension(&mut self, extension: impl ExtensionTrait) {
        self.storage
            .entry(extension.get_object())
            .or_default()
            .insert(extension.to_string());
    }

    /// Build extension query parameters draining this storage
    pub fn build_query_drain(&mut self, request_context: &mut RequestContext) {
        for (object, extend) in self.storage.drain() {
            request_context.query.push((
                format!("extend[{}]", object),
                extend.into_iter().collect::<Vec<_>>().join(","),
            ));
        }
    }
}
