//! Genre

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::ResourceHeader;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Genre
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Genre attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<GenreAttributes>,
}

impl Genre {
    /// Get genre request builder
    pub fn get<'a>() -> GenreGetRequestBuilder<'a> {
        GenreGetRequestBuilder::default()
    }
}

/// Genre attributes
#[derive(ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(GenreAttributesExtension, object = "genres", extension, whitelist)]
pub struct GenreAttributes {
    /// The localized name of the genre
    pub name: String,
    /// The identifier of the parent for the genre
    pub parent_id: Option<String>,
    /// The localized name of the parent genre
    pub parent_name: Option<String>,
    /// **(Extended)** A localized string to use when displaying the genre in relation to charts
    #[resource_property(whitelist, name = "chartLabel")]
    pub chart_label: Option<String>,
}

/// Genre request builder
pub struct GenreRequestBuilder;

/// Genre get request builder
pub type GenreGetRequestBuilder<'a> = MusicRequestBuilder<'a, GenreRequestBuilder>;

impl<'a> GenreGetRequestBuilder<'a> {
    /// Fetch one genre by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Genre>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/genres/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let response = try_resource_response(response).await?;
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple genres by id
    pub async fn many(mut self, client: &ApiClient, ids: &[&str]) -> Result<Vec<Genre>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/genres",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let response = try_resource_response(response).await?;
        Ok(response.data)
    }

    /// Fetch all genres for the current top charts    
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    pub async fn top_charts(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<Genre, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            format!(
                "/v1/catalog/{storefront}/genres",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ),
            request_context,
            offset,
        )
    }
}
