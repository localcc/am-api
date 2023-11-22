//! Playlist

use crate::error::Error;
use crate::primitive::{PlayParameters, TrackType};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::attributes::{DescriptionAttribute, TitleOnlyAttribute};
use crate::resource::catalog::artist::Artist;
use crate::resource::library::playlist::LibraryPlaylist;
use crate::resource::relationship::Relationship;
use crate::resource::view::View;
use crate::resource::{Resource, ResourceHeader};
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;

/// Playlist
#[derive(Context, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Playlist attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<PlaylistAttributes>,
    /// Playlist relationships
    #[serde(default)]
    pub relationships: PlaylistRelationships,
    /// The views for associations between playlists and other resources
    #[serde(default)]
    pub views: PlaylistViews,
}

impl Playlist {
    /// Get playlist request builder
    pub fn get<'a>() -> PlaylistGetRequestBuilder<'a> {
        PlaylistGetRequestBuilder::default()
    }
}

/// Playlist attributes
#[derive(ResourceProperty, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[resource_property(
    PlaylistAttributesExtension,
    object = "playlists",
    extension,
    whitelist
)]
pub struct PlaylistAttributes {
    /// Playlist artwork
    #[serde(default)]
    pub artwork: Option<Artwork>,
    /// The display name of the curator
    pub curator_name: String,
    /// A description of the playlist
    #[serde(default)]
    pub description: Option<DescriptionAttribute>,
    /// Indicates whether the playlist represents a popularity chart
    pub is_chart: bool,
    /// The date the playlist was last modified
    #[serde(with = "time::serde::iso8601")]
    pub last_modified_date: OffsetDateTime,
    /// The localized name of the playlist
    pub name: String,
    /// The type of playlist
    pub playlist_type: PlaylistType,
    /// The parameters to use to play back the tracks in the playlist
    #[serde(default)]
    pub play_params: Option<PlayParameters>,
    /// The URL for sharing the playlist in Apple Music
    pub url: String,
    /// **(Extended)** The resource types that are present in the tracks of the playlists
    #[resource_property(whitelist, name = "trackTypes")]
    #[serde(default)]
    pub track_types: Option<Vec<TrackType>>,
}

/// Playlist relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(PlaylistRelationshipType, object = "playlists", relationship)]
pub struct PlaylistRelationships {
    /// The curator that created the playlist. By default, curator includes identifiers only.
    ///
    /// Fetch limits: None
    ///
    /// Possible resources: [`Activity`], [`AppleCurator`], [`Curator`]
    pub curator: Option<Relationship<Resource>>,
    /// Library playlist for a catalog playlist if added to library.
    ///
    /// Possible resources: [`LibraryPlaylist`]
    pub library: Option<Relationship<LibraryPlaylist>>,
    /// The songs and music videos included in the playlist. By default, tracks includes objects.
    ///
    /// Fetch limits: 100 default, 300 maximum
    ///
    /// Possible resources: [`MusicVideo`], [`Song`]
    pub tracks: Option<Relationship<Resource>>,
}

/// Playlist views
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(PlaylistViewType, object = "playlists", view)]
pub struct PlaylistViews {
    /// Artists that are featured on this playlist
    ///
    /// Possible resources: [`Artist`]
    #[serde(rename = "featured-artists")]
    pub featured_artists: Option<View<TitleOnlyAttribute, Artist>>,
    /// Additional content by the same curator for this playlist
    ///
    /// Possible resources: [`Playlist`]
    #[serde(rename = "more-by-curator")]
    pub more_by_curator: Option<View<TitleOnlyAttribute, Playlist>>,
}

/// Playlist type
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlaylistType {
    /// A playlist created by an Apple Music curator
    #[serde(rename = "editorial")]
    Editorial,
    /// A playlist created by a non-Apple curator or brand
    #[serde(rename = "external")]
    External,
    /// A personalized playlist for an Apple Music user
    #[serde(rename = "personal-mix")]
    PersonalMix,
    /// A personalized Apple Music Replay playlist for an Apple Music user
    #[serde(rename = "replay")]
    Replay,
    /// A playlist created and shared by an Apple Music user
    #[serde(rename = "user-shared")]
    #[default]
    UserShared,
}

/// Playlist request builder
pub struct PlaylistRequestBuilder;

/// Playlist get request builder
pub type PlaylistGetRequestBuilder<'a> = MusicRequestBuilder<'a, PlaylistRequestBuilder>;

impl<'a> PlaylistGetRequestBuilder<'a> {
    /// Fetch one playlist by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Playlist>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/playlists/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch many playlists by id
    pub async fn many(mut self, client: &ApiClient, ids: &[&str]) -> Result<Vec<Playlist>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/playlists",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch chart playlists by storefront value
    pub async fn chart(
        mut self,
        client: &ApiClient,
        storefront: &str,
    ) -> Result<Vec<Playlist>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context.query.push((
            String::from("filter[storefront-chart]"),
            storefront.to_string(),
        ));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/playlists",
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
