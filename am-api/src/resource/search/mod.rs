//! Apple music search

use crate::error::Error;
use crate::request::context::{ContextContainer, RequestContext};
use crate::resource::ErrorResponse;
use crate::ApiClient;
use async_stream::try_stream;
use futures::Stream;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub mod catalog;
pub mod library;

/// Search relationship
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchRelationship<T> {
    /// A relative location for the relationship
    #[serde(default)]
    pub href: Option<String>,
    /// A relative cursor to fetch the next paginated collection of resources in the relationship if more exist
    #[serde(default)]
    pub next: Option<String>,
    /// Associated data
    #[serde(default = "Vec::default")]
    pub data: Vec<T>,
    /// Context
    #[serde(skip, default)]
    context: Option<Arc<RequestContext>>,
}

impl<T> SearchRelationship<T>
where
    T: Clone + DeserializeOwned + ContextContainer,
{
    /// Iterate this relationship
    pub fn iter(&self, client: &ApiClient) -> impl Stream<Item = Result<T, Error>> {
        let relationship = self.clone();
        let client = client.clone();
        let context = relationship
            .context
            .clone()
            .expect("context should always exist on relationships");

        try_stream! {
            let mut relationship = Some(relationship);

            while let Some(rel) = relationship {
                for mut entry in rel.data {
                    entry.set_context(context.clone());
                    yield entry;
                }

                let Some(next) = rel.next.as_ref() else {
                    return;
                };

                let response = client.get(next.as_str()).query(&context.query).send().await?;
                relationship = Self::try_relationship_response(response).await?;
            }
        }
    }

    async fn try_relationship_response(response: Response) -> Result<Option<Self>, Error> {
        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        let result: Value = response.json().await?;
        result
            .as_object()
            .and_then(|e| e.get("results"))
            .and_then(|e| e.as_object())
            .and_then(|e| e.values().next())
            .map(|e| serde_json::from_value(e.clone()))
            .transpose()
            .map_err(|e| e.into())
    }
}

impl<T> ContextContainer for SearchRelationship<T>
where
    T: ContextContainer,
{
    fn set_context(&mut self, context: Arc<RequestContext>) {
        self.context = Some(context.clone());
        self.data.set_context(context.clone());
    }
}

impl<T> Debug for SearchRelationship<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchRelationship")
            .field("href", &self.href)
            .field("next", &self.next)
            .field("data", &self.data)
            .finish()
    }
}

impl<T> PartialEq for SearchRelationship<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.href == other.href && self.next == other.next && self.data == other.data
    }
}

impl<T> Eq for SearchRelationship<T> where T: PartialEq + Eq {}

impl<T> Hash for SearchRelationship<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.href.hash(state);
        self.next.hash(state);
        self.data.hash(state);
    }
}

impl<T> Default for SearchRelationship<T> {
    fn default() -> Self {
        SearchRelationship {
            href: None,
            next: None,
            data: Vec::default(),
            context: None,
        }
    }
}
