//! Pagination cursor

use crate::error::Error;
use crate::request::context::{ContextContainer, RequestContext};
use crate::request::try_resource_response;
use crate::ApiClient;
use async_stream::try_stream;
use futures::Stream;
use serde::de::DeserializeOwned;
use std::sync::Arc;

/// Paginate a request
pub(crate) fn paginate<R>(
    client: ApiClient,
    endpoint: String,
    mut request_context: RequestContext,
    mut offset: usize,
) -> impl Stream<Item = Result<R, Error>>
where
    R: ContextContainer + DeserializeOwned,
{
    try_stream! {
        loop {
            request_context.query.push((String::from("offset"), offset.to_string()));

            let response = client
                .get(&endpoint)
                .query(&request_context.query)
                .send()
                .await?;

            request_context.query.pop();

            let mut response = try_resource_response(response).await?;
            response.data.set_context(Arc::new(request_context.clone()));

            offset += response.data.len();

            if response.data.is_empty() {
                return;
            }

            for resource in response.data {
                yield resource;
            }

        }
    }
}
