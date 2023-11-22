//! Record label

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::attributes::{DescriptionAttribute, TitleOnlyAttribute};
use crate::resource::catalog::album::Album;
use crate::resource::view::View;
use crate::resource::ResourceHeader;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Record label
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RecordLabel {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Record label attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<RecordLabelAttributes>,
    /// The relationship views for the record label
    #[serde(default)]
    pub views: RecordLabelViews,
}

impl RecordLabel {
    /// Get record label request builder
    pub fn get<'a>() -> RecordLabelGetRequestBuilder<'a> {
        RecordLabelGetRequestBuilder::default()
    }
}

/// Record label attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct RecordLabelAttributes {
    /// Artwork associated with this content
    pub artwork: Artwork,
    /// A map of description information
    pub description: Option<DescriptionAttribute>,
    /// The (potentially) censored name of the content
    pub name: String,
    /// The URL to load the record label from
    pub url: String,
}

/// Record label views
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[resource_property(RecordLabelViewType, object = "record-labels", view)]
pub struct RecordLabelViews {
    /// The latest releases for the record label
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "latest-releases")]
    pub latest_releases: Option<View<TitleOnlyAttribute, Album>>,
    /// The top releases for the record label
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "top-releases")]
    pub top_releases: Option<View<TitleOnlyAttribute, Album>>,
}

/// Record label request builder
pub struct RecordLabelRequestBuilder;

/// Record label get request builder
pub type RecordLabelGetRequestBuilder<'a> = MusicRequestBuilder<'a, RecordLabelRequestBuilder>;

impl<'a> RecordLabelGetRequestBuilder<'a> {
    /// Fetch one record label by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<RecordLabel>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/record-labels/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch many record labels by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<RecordLabel>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/record-labels",
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
