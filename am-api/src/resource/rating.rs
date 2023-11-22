//! Rating

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::relationship::Relationship;
use crate::resource::{ErrorResponse, Resource, ResourceHeader, ResourceInfo, ResourceType};
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::sync::Arc;

/// Rating
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Rating attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<RatingAttributes>,
    /// Rating relationships
    pub relationships: RatingRelationships,
}

impl Rating {
    /// Get rating request builder
    pub fn get<'a>() -> RatingGetRequestBuilder<'a> {
        RatingGetRequestBuilder::default()
    }

    /// Post rating request builder
    pub fn add_rating<'a>() -> RatingPostRequestBuilder<'a> {
        RatingPostRequestBuilder::default()
    }

    /// Post rating request builder
    pub fn remove_rating<'a>() -> RatingPostRequestBuilder<'a> {
        RatingPostRequestBuilder::default()
    }
}

/// Rating attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct RatingAttributes {
    /// The value for the resource’s rating. The possible values for the value key are 1 and -1. If a value isn’t present, the content doesn’t have a rating
    pub rating: Option<i32>,
}

/// Rating relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(RatingRelationshipType, object = "ratings", relationship)]
pub struct RatingRelationships {
    /// The content associated with the rating.
    ///
    /// Fetch limits: None.
    ///
    /// Posssible resources: [`Album`], [`LibraryMusicVideo`], [`LibraryPlaylist`], [`LibrarySong`], [`MusicVideo`], [`Playlist`], [`Song`], [`Station`]
    pub content: Option<Relationship<Resource>>,
}

/// Rating type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RatingType {
    /// Album
    Album,
    /// Music video
    MusicVideo,
    /// Playlist
    Playlist,
    /// Song
    Song,
    /// Station
    Station,
    /// Library album
    LibraryAlbum,
    /// Library music video
    LibraryMusicVideo,
    /// Library playlist
    LibraryPlaylist,
    /// Library song
    LibrarySong,
}

impl std::fmt::Display for RatingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let endpoint = match self {
            RatingType::Album => "albums",
            RatingType::MusicVideo => "music-videos",
            RatingType::Playlist => "playlists",
            RatingType::Song => "songs",
            RatingType::Station => "stations",
            RatingType::LibraryAlbum => "library-albums",
            RatingType::LibraryMusicVideo => "library-music-videos",
            RatingType::LibraryPlaylist => "library-playlists",
            RatingType::LibrarySong => "library-songs",
        };
        write!(f, "{}", endpoint)
    }
}

/// Rating request builder marker
pub struct RatingGetRequestBuilderMarker;

/// Rating get request builder
pub type RatingGetRequestBuilder<'a> = MusicRequestBuilder<'a, RatingGetRequestBuilderMarker>;

impl<'a> RatingGetRequestBuilder<'a> {
    /// Fetch one rating for a resource
    pub async fn one(
        mut self,
        client: &ApiClient,
        rating_type: RatingType,
        id: &str,
    ) -> Result<Option<Rating>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let endpoint = rating_type.to_string();

        let response = client
            .get(&format!("/v1/me/ratings/{endpoint}/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple ratings by ids
    pub async fn many(
        mut self,
        client: &ApiClient,
        rating_type: RatingType,
        ids: &[&str],
    ) -> Result<Vec<Rating>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let endpoint = rating_type.to_string();

        let response = client
            .get(&format!("/v1/me/ratings/{endpoint}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }
}

/// Rating post request builder marker
pub struct RatingPostRequestBuilderMarker;

/// Rating post request builder
pub type RatingPostRequestBuilder<'a> = MusicRequestBuilder<'a, RatingPostRequestBuilderMarker>;

impl<'a> RatingPostRequestBuilder<'a> {
    /// Add a rating to a resource
    ///
    /// # Return value
    ///
    /// Resource of type [`Rating`]
    pub async fn add_rating(
        mut self,
        client: &ApiClient,
        resource: &Resource,
    ) -> Result<Option<Rating>, Error> {
        Self::check_supported(resource)?;

        let request_context = Arc::new(self.get_request_context_drain(client));
        let endpoint = resource.get_type();

        let response = client
            .put(&format!(
                "/v1/me/ratings/{endpoint}/{id}",
                id = resource.get_header().id
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Remove a rating from a resource
    pub async fn remove_rating(
        mut self,
        client: &ApiClient,
        resource: &Resource,
    ) -> Result<(), Error> {
        Self::check_supported(resource)?;

        let request_context = Arc::new(self.get_request_context_drain(client));
        let endpoint = resource.get_type();

        let response = client
            .delete(&format!(
                "/v1/me/ratings/{endpoint}/{id}",
                id = resource.get_header().id
            ))
            .query(&request_context.query)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        Ok(())
    }

    /// Check if the passed in resource is supported
    fn check_supported(resource: &Resource) -> Result<(), Error> {
        let supported = matches!(
            resource,
            Resource::Album { .. }
                | Resource::MusicVideo { .. }
                | Resource::Playlist { .. }
                | Resource::Song { .. }
                | Resource::Station { .. }
                | Resource::LibraryAlbum { .. }
                | Resource::LibraryMusicVideo { .. }
                | Resource::LibraryPlaylist { .. }
                | Resource::LibrarySong { .. }
        );

        match supported {
            true => Ok(()),
            false => Err(Error::InvalidResourceType),
        }
    }
}
