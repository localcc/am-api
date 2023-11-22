//! Apple music library

use crate::error::Error;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::{Resource, ResourceInfo, ResourceType};
use crate::ApiClient;

use crate::request::builder::MusicRequestBuilder;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub mod album;
pub mod artist;
pub mod music_video;
pub mod playlist;
pub mod search;
pub mod song;

/// Library builder
pub struct LibraryBuilder;

/// Library add resource builder
pub type LibraryAddResourceBuilder<'a> =
    MusicRequestBuilder<'a, LibraryBuilder, HashMap<&'static str, HashSet<String>>>;

impl<'a> LibraryAddResourceBuilder<'a> {
    /// Create a new [`LibraryAddResourceBuilder`] instance
    pub fn new() -> LibraryAddResourceBuilder<'a> {
        LibraryAddResourceBuilder::default()
    }

    /// Add a resource to the library
    pub fn add_resource(mut self, resource: &Resource) -> Result<Self, Error> {
        let supported = matches!(
            resource,
            Resource::Album { .. }
                | Resource::Artist { .. }
                | Resource::MusicVideo { .. }
                | Resource::Playlist { .. }
                | Resource::Song { .. }
        );

        if !supported {
            return Err(Error::InvalidResourceType);
        }
        self.data
            .entry(resource.get_type())
            .or_default()
            .insert(resource.get_header().id.clone());

        Ok(self)
    }

    /// Send the request
    pub async fn send(mut self, client: &ApiClient) -> Result<Vec<Resource>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("representation"), String::from("ids")));

        for (resource_type, ids) in self.data {
            request_context.query.push((
                format!("ids[{}]", resource_type),
                ids.into_iter().collect::<Vec<_>>().join(","),
            ));
        }

        let request_context = Arc::new(request_context);

        let response = client
            .post("/v1/me/library")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }
}
