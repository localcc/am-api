//! Activity

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

/// Activity
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Activity attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<ActivityAttributes>,
    /// Activity relationships
    #[serde(default)]
    pub relationships: ActivityRelationships,
}

impl Activity {
    /// Activity get request builder
    pub fn get<'a>() -> ActivityGetRequestBuilder<'a> {
        ActivityGetRequestBuilder::default()
    }
}

/// Activity attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct ActivityAttributes {
    /// The activity artwork
    pub artwork: Artwork,
    /// The notes about the activity that appear in the Apple Music catalog
    pub editorial_notes: Option<EditorialNotes>,
    /// The localized name of the activity
    pub name: String,
    /// The URL for sharing the activity in Apple Music
    pub url: String,
}

/// Activity relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(ActivityRelationshipType, object = "activities", relationship)]
pub struct ActivityRelationships {
    /// Playlists
    ///
    /// Possible resources: [`Playlist`]
    pub playlists: Option<Relationship<Playlist>>,
}

/// Activity request builder
pub struct ActivityRequestBuilder;

/// Activity get request builder
pub type ActivityGetRequestBuilder<'a> = MusicRequestBuilder<'a, ActivityRequestBuilder>;

impl<'a> ActivityGetRequestBuilder<'a> {
    /// Fetch one catalog activity by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Activity>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));
        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/activities/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple activities by id
    pub async fn many(mut self, client: &ApiClient, ids: &[&str]) -> Result<Vec<Activity>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/activities",
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
