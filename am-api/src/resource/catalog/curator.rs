//! Curators

use crate::error::Error;
use crate::primitive::EditorialNotes;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::catalog::playlist::Playlist;
use crate::resource::relationship::Relationship;
use crate::resource::ResourceHeader;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Apple curator
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AppleCurator {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Apple curator attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<AppleCuratorAttributes>,
    /// Apple curator relationships
    #[serde(default)]
    pub relationships: AppleCuratorRelationships,
}

impl AppleCurator {
    /// Get apple curator request builder
    pub fn get<'a>() -> AppleCuratorGetRequestBuilder<'a> {
        AppleCuratorGetRequestBuilder::default()
    }
}

/// Apple curator attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct AppleCuratorAttributes {
    /// The curator artwork
    pub artwork: Artwork,
    /// The notes about the curator that appear in the Apple Music catalog
    pub editorial_notes: Option<EditorialNotes>,
    /// Curator kind
    pub kind: CuratorKind,
    /// The localized name of the curator
    pub name: String,
    /// The localized shortened name of the curator
    pub short_name: Option<String>,
    /// The name of the host if kind is Show
    pub show_host_name: Option<String>,
    /// The URL for sharing the curator in Apple Music
    pub url: String,
}

/// Curator kind
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CuratorKind {
    /// An individual curator entity
    #[default]
    Curator,
    /// A curator that represents a cohesive music genre
    Genre,
    /// A curator associated with a particular Apple Music show
    Show,
}

/// Apple curator relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(AppleCuratorRelationshipType, object = "apple-curators", relationship)]
pub struct AppleCuratorRelationships {
    /// The playlists associated with this curator. By default, playlists includes identifiers only.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`Playlist`]
    pub playlists: Option<Relationship<Playlist>>,
}

/// Apple curator request builder
pub struct AppleCuratorRequestBuilder;

/// Apple curator get request builder
pub type AppleCuratorGetRequestBuilder<'a> = MusicRequestBuilder<'a, AppleCuratorRequestBuilder>;

impl<'a> AppleCuratorGetRequestBuilder<'a> {
    /// Fetch one apple curator by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<AppleCurator>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/apple-curators/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple apple curators by id
    pub async fn main(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<AppleCurator>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/apple-curators",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }
}

/// Curator
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Curator {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Curator attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<CuratorAttributes>,
    /// Curator relationships
    #[serde(default)]
    pub relationships: CuratorRelationships,
}

impl Curator {
    /// Get curator request builder
    pub fn get<'a>() -> CuratorGetRequestBuilder<'a> {
        CuratorGetRequestBuilder::default()
    }
}

/// Curator attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct CuratorAttributes {
    /// The curator artwork
    pub artwork: Artwork,
    /// The notes about the curator
    pub editorial_notes: Option<EditorialNotes>,
    /// The localized name of the curator
    pub name: String,
    /// The URL for sharing the curator in Apple Music
    pub url: String,
}

/// Curator relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(CuratorRelationshipType, object = "curators", relationship)]
pub struct CuratorRelationships {
    /// The playlists associated with this curator. By default, playlists includes identifiers only.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`Playlist`]
    pub playlists: Option<Relationship<Playlist>>,
}

/// Curator request builder
pub struct CuratorRequestBuilder;

/// Curator get request builder
pub type CuratorGetRequestBuilder<'a> = MusicRequestBuilder<'a, CuratorRequestBuilder>;

impl<'a> CuratorGetRequestBuilder<'a> {
    /// Fetch one curator by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Curator>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/curators/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple curators by id
    pub async fn many(mut self, client: &ApiClient, ids: &[&str]) -> Result<Vec<Curator>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/curators",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }
}
