//! Request builder
use crate::request::context::RequestContext;
use crate::request::extension::{ExtensionStorage, ExtensionTrait};
use crate::request::relationship::{RelationshipStorage, RelationshipTrait};
use crate::request::view::{ViewStorage, ViewTrait};
use crate::ApiClient;
use std::marker::PhantomData;

/// A request builder
pub struct MusicRequestBuilder<'a, BuilderType, Data = ()> {
    /// Storefront
    pub(crate) storefront_override: Option<celes::Country>,
    /// Localization
    pub(crate) localization_override: Option<&'a str>,
    /// Extensions
    pub(crate) extensions: ExtensionStorage,
    /// Relationships
    pub(crate) relationships: RelationshipStorage,
    /// Views
    pub(crate) views: ViewStorage,
    /// Data
    pub(crate) data: Data,
    pub(crate) _marker: PhantomData<BuilderType>,
}

impl<'a, BuilderType, Data> MusicRequestBuilder<'a, BuilderType, Data> {
    /// Override storefront for this request
    pub fn override_storefront(mut self, storefront: celes::Country) -> Self {
        self.storefront_override = Some(storefront);
        self
    }

    /// Override localization for this request
    pub fn override_localization(mut self, localization: &'a str) -> Self {
        self.localization_override = Some(localization);
        self
    }

    /// Extend resource attributes
    pub fn extend(mut self, extension: impl ExtensionTrait) -> Self {
        self.extensions.add_extension(extension);
        self
    }

    /// Include a relationship
    ///
    /// This will fetch the full object for the relationship data
    pub fn include(mut self, relationship: impl RelationshipTrait) -> Self {
        self.relationships.add_relationship(relationship);
        self
    }

    /// Lazily include a relationship
    ///
    /// This will only fetch the identifiers for the relationship data
    pub fn include_lazy(mut self, relationship: impl RelationshipTrait) -> Self {
        self.relationships.add_relationship_lazy(relationship);
        self
    }

    /// Add a relationship view
    pub fn view(mut self, view: impl ViewTrait) -> Self {
        self.views.add_view(view);
        self
    }

    /// Get request context draining this builder
    pub(crate) fn get_request_context_drain(&mut self, client: &ApiClient) -> RequestContext {
        let storefront = self
            .storefront_override
            .unwrap_or(client.get_storefront_country());

        let localization = self
            .localization_override
            .unwrap_or(client.get_localization());

        let mut context = RequestContext {
            storefront,
            query: Vec::from([(String::from("l"), localization.to_string())]),
        };

        self.extensions.build_query_drain(&mut context);
        self.relationships.build_query_drain(&mut context);
        self.views.build_query_drain(&mut context);

        context
    }
}

impl<'a, T, Data> Default for MusicRequestBuilder<'a, T, Data>
where
    Data: Default,
{
    fn default() -> Self {
        MusicRequestBuilder {
            storefront_override: None,
            localization_override: None,
            extensions: Default::default(),
            relationships: Default::default(),
            views: Default::default(),
            data: Default::default(),
            _marker: Default::default(),
        }
    }
}
