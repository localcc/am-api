//! Album

use crate::error::Error;
use crate::primitive::{AudioVariant, ContentRating, EditorialNotes, PlayParameters};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::attributes::TitleOnlyAttribute;
use crate::resource::catalog::artist::Artist;
use crate::resource::catalog::music_video::MusicVideo;
use crate::resource::catalog::playlist::Playlist;
use crate::resource::catalog::record_label::RecordLabel;
use crate::resource::genre::Genre;
use crate::resource::library::album::LibraryAlbum;
use crate::resource::relationship::Relationship;
use crate::resource::view::View;
use crate::resource::{Resource, ResourceHeader};
use crate::time::year_or_date::YearOrDate;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Album
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Album attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<AlbumAttributes>,
    /// Album relationships
    #[serde(default)]
    pub relationships: AlbumRelationships,
    /// The relationship views for the album
    #[serde(default)]
    pub views: AlbumViews,
}

impl Album {
    /// Get album request builder
    pub fn get<'a>() -> AlbumGetRequestBuilder<'a> {
        AlbumGetRequestBuilder::default()
    }
}

/// Album attributes
#[derive(ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(AlbumAttributesExtension, object = "albums", extension, whitelist)]
pub struct AlbumAttributes {
    /// The name of the primary artist associated with the album
    pub artist_name: String,
    /// **(Extended)** The URL of the artist for this content
    #[resource_property(whitelist, name = "artistUrl")]
    pub artist_url: Option<String>,
    /// The artwork for the album
    pub artwork: Artwork,
    /// Specific audio variants for an album
    pub audio_variants: Option<Vec<AudioVariant>>,
    /// The Recording Industry Association of America (RIAA) rating of the content. No value means no rating
    pub content_rating: Option<ContentRating>,
    /// The copyright text
    pub copyright: String,
    /// The notes about the album that appear in the iTunes Store
    pub editorial_notes: Option<EditorialNotes>,
    /// Genre names
    pub genre_names: Vec<String>,
    /// Indicates whether the album is marked as a compilation
    pub is_compilation: bool,
    /// Indicates whether the album is complete. If true, the album is complete; otherwise, it's not. An album is complete if it contains all its tracks and songs
    pub is_complete: bool,
    /// Indicates whether the response delivered the album as an Apple Digital Master
    pub is_mastered_for_itunes: bool,
    /// Indicates whether the album contains a single song
    pub is_single: bool,
    /// The localized name of the album
    pub name: String,
    /// The parameters to use to play back the tracks of the album
    pub play_params: Option<PlayParameters>,
    /// The name of the record label for the album
    pub record_label: Option<String>,
    /// The release date of the album, when known, in YYYY-MM-DD or YYYY format. Prerelease content may have an expected release date in the future
    pub release_date: Option<YearOrDate>,
    /// The number of tracks for the album
    pub track_count: u32,
    /// The Universal Product Code for the album
    pub upc: Option<String>,
    /// The URL for sharing the album in Apple Music
    pub url: String,
}

/// Album relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(AlbumRelationshipType, object = "albums", relationship)]
pub struct AlbumRelationships {
    /// The artists associated with the album. By default, artists includes identifiers only.
    ///
    /// Fetch limits: 10 default, 10 maximum
    ///
    /// Possible resources: [`Artist`]
    pub artists: Option<Relationship<Artist>>,
    /// The genres for the album. By default, genres not included.
    ///
    /// Fetch limits: None
    ///
    /// Possible resources: [`Genre`]
    pub genres: Option<Relationship<Genre>>,
    /// The songs and music videos on the album. By default, tracks includes objects.
    ///
    /// Fetch limits: 300 default, 300 maximum
    ///
    /// Possible resources: [`MusicVideo`], [`Song`]
    pub tracks: Option<Relationship<Resource>>,
    /// The album in the user’s library for the catalog album, if any.
    ///
    /// Fetch limits: None
    ///
    /// Possible resources: [`LibraryAlbum`]
    pub library: Option<Relationship<LibraryAlbum>>,
    /// The record labels for the album
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`RecordLabel`]
    #[serde(rename = "record-labels")]
    pub record_labels: Option<Relationship<RecordLabel>>,
}

/// Album views
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(AlbumViewType, object = "albums", view)]
pub struct AlbumViews {
    /// A selection of playlists that tracks from this album appear on
    ///
    /// Possible resources: [`Playlist`]
    #[serde(rename = "appears-on")]
    pub appears_on: Option<View<TitleOnlyAttribute, Playlist>>,
    /// Other versions of this album
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "other-versions")]
    pub other_versions: Option<View<TitleOnlyAttribute, Album>>,
    /// Other albums related or similar to this album
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "related-albums")]
    pub related_albums: Option<View<TitleOnlyAttribute, Album>>,
    /// Music videos associated with tracks on this album
    ///
    /// Possible resources: [`MusicVideo`]
    #[serde(rename = "related-videos")]
    pub related_videos: Option<View<TitleOnlyAttribute, MusicVideo>>,
}

/// Album request builder
pub struct AlbumRequestBuilder;

/// Album get request builder
pub type AlbumGetRequestBuilder<'a> = MusicRequestBuilder<'a, AlbumRequestBuilder>;

impl<'a> AlbumGetRequestBuilder<'a> {
    /// Fetch one album by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Album>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));
        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/albums/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple albums by id
    ///
    /// # Params
    ///
    /// * upc - if the ids are UPCs or album ids, false means album ids, true means UPCs
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
        upc: bool,
    ) -> Result<Vec<Album>, Error> {
        let mut request_context = self.get_request_context_drain(client);

        let ids = ids.to_vec().join(",");
        let id_query = match upc {
            true => "filter[upc]",
            false => "ids",
        };
        request_context.query.push((id_query.to_string(), ids));

        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/albums",
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
