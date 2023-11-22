//! Apple music view

use crate::error::Error;
use crate::request::context::{ContextContainer, RequestContext};
use crate::resource::ErrorResponse;
use crate::ApiClient;
use async_stream::try_stream;
use futures::Stream;
use reqwest::{Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Apple music view
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct View<Attributes, T> {
    /// The relative location to fetch the view directly
    #[serde(default)]
    pub href: Option<String>,
    /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching
    #[serde(default)]
    pub next: Option<String>,
    /// Attributes
    pub attributes: Attributes,
    /// Data
    #[serde(default = "Vec::default")]
    pub data: Vec<T>,
    /// Context
    #[serde(skip, default)]
    context: Option<Arc<RequestContext>>,
}

impl<Attributes, T> View<Attributes, T>
where
    Attributes: Clone + DeserializeOwned,
    T: Clone + DeserializeOwned + ContextContainer,
{
    /// Iterate this view
    pub fn iter(&self, client: &ApiClient) -> impl Stream<Item = Result<T, Error>> {
        let view = self.clone();
        let client = client.clone();
        let context = view
            .context
            .clone()
            .expect("context should always exist on views");

        try_stream! {
            let mut view = view;

            loop {
                for mut entry in view.data {
                    entry.set_context(context.clone());
                    yield entry;
                }

                let Some(next) = view.next.as_ref() else {
                    return;
                };

                let response = client.get(next.as_str()).query(&context.query).send().await?;
                view = Self::try_view_response(response).await?;
            }
        }
    }

    async fn try_view_response(response: Response) -> Result<Self, Error> {
        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        let result = response.json().await?;
        Ok(result)
    }
}

impl<Attributes, T> ContextContainer for View<Attributes, T>
where
    T: ContextContainer,
{
    fn set_context(&mut self, context: Arc<RequestContext>) {
        self.context = Some(context.clone());
        self.data.set_context(context)
    }
}

impl<Attributes, T> Debug for View<Attributes, T>
where
    Attributes: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("View")
            .field("href", &self.href)
            .field("next", &self.next)
            .field("attributes", &self.attributes)
            .field("data", &self.data)
            .finish()
    }
}

impl<Attributes, T> PartialEq for View<Attributes, T>
where
    Attributes: PartialEq,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.href == other.href
            && self.next == other.next
            && self.attributes == other.attributes
            && self.data == other.data
    }
}

impl<Attributes, T> Eq for View<Attributes, T>
where
    Attributes: Eq,
    T: Eq,
{
}

impl<Attributes, T> Hash for View<Attributes, T>
where
    Attributes: Hash,
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.href.hash(state);
        self.next.hash(state);
        self.attributes.hash(state);
        self.data.hash(state);
    }
}
