//! Storefront

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::ResourceHeader;
use crate::ApiClient;
use am_api_proc_macro::Context;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Storefront
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Storefront {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Storefront attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<StorefrontAttributes>,
}

impl Storefront {
    /// Get storefront request builder
    pub fn get<'a>() -> StorefrontGetRequestBuilder<'a> {
        StorefrontGetRequestBuilder::default()
    }
}

/// Storefront attributes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StorefrontAttributes {
    /// The default supported RFC4646 language tag for the storefront
    pub default_language_tag: String,
    /// Attribute indicating the level that this storefront can display explicit content
    pub explicit_content_policy: ExplicitContentPolicy,
    /// The localized name of the storefront
    pub name: String,
    /// The supported RFC4646 language tags for the storefront
    pub supported_language_tags: Vec<String>,
}

/// Storefront explicit content policy
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExplicitContentPolicy {
    /// Allowed
    #[serde(rename = "allowed")]
    Allowed,
    /// Opt-in
    #[serde(rename = "opt-in")]
    OptIn,
    /// Prohibited
    #[serde(rename = "prohibited")]
    Prohibited,
}

/// Storefront get request builder marker
pub struct StorefrontGetRequestBuilderMarker;

/// Storefront get request builder
pub type StorefrontGetRequestBuilder<'a> =
    MusicRequestBuilder<'a, StorefrontGetRequestBuilderMarker>;

impl<'a> StorefrontGetRequestBuilder<'a> {
    /// Fetch one storefront using a country
    pub async fn one(
        mut self,
        client: &ApiClient,
        country: celes::Country,
    ) -> Result<Option<Storefront>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));
        let response = client
            .get(&format!(
                "/v1/storefronts/{}",
                country.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple storefronts using countries
    ///
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    pub async fn many(
        mut self,
        client: &ApiClient,
        countries: &[celes::Country],
    ) -> Result<Vec<Storefront>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context.query.push((
            String::from("ids"),
            countries
                .iter()
                .map(|e| e.alpha2.to_lowercase())
                .collect::<Vec<_>>()
                .join(","),
        ));
        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/storefronts")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch all storefronts
    pub fn all(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<Storefront, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/storefronts"),
            request_context,
            offset,
        )
    }
}
