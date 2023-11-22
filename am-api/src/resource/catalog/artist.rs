//! Artist

use crate::error::Error;
use crate::primitive::EditorialNotes;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::attributes::TitleOnlyAttribute;
use crate::resource::catalog::album::Album;
use crate::resource::catalog::music_video::MusicVideo;
use crate::resource::catalog::playlist::Playlist;
use crate::resource::catalog::song::Song;
use crate::resource::catalog::station::Station;
use crate::resource::genre::Genre;
use crate::resource::relationship::Relationship;
use crate::resource::view::View;
use crate::resource::ResourceHeader;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Artist
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Artist attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<ArtistAttributes>,
    /// Artist relationships
    #[serde(default)]
    pub relationships: ArtistRelationships,
    /// The views for associations between artists and other resources
    #[serde(default)]
    pub views: ArtistViews,
}

impl Artist {
    /// Get artist request builder
    pub fn get<'a>() -> ArtistGetRequestBuilder<'a> {
        ArtistGetRequestBuilder::default()
    }
}

/// Artist attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct ArtistAttributes {
    /// The artwork for the artist image
    pub artwork: Option<Artwork>,
    /// The notes about the artist that appear in the Apple Music catalog
    pub editorial_notes: Option<EditorialNotes>,
    /// The names of the genres associated with this artist
    #[serde(rename = "genreNames")]
    pub genres: Vec<String>,
    /// The localized name of the artist
    pub name: String,
    /// The URL for sharing the artist in Apple Music
    pub url: String,
}

/// Artist relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(ArtistRelationshipType, object = "artists", relationship)]
pub struct ArtistRelationships {
    /// The albums associated with the artist. By default, albums includes identifiers only.
    ///
    /// Fetch limits: 25 default, 100 maximum
    ///
    /// Possible resources: [`Album`]
    pub albums: Option<Relationship<Album>>,
    /// The genres associated with the artist. By default, genres not included.
    ///
    /// Fetch limits: None
    ///
    /// Possible resources: [`Genre`]
    pub genres: Option<Relationship<Genre>>,
    /// The music videos associated with the artist. By default, musicVideos not included.
    ///
    /// Fetch limits: 25 default, 100 maximum
    ///
    /// Possible resources: [`MusicVideo`]
    #[serde(rename = "music-videos")]
    pub music_videos: Option<Relationship<MusicVideo>>,
    /// The playlists associated with the artist. By default, playlists not included.
    ///
    /// Fetch limits: 10 default, 10 maximum
    ///
    /// Possible resources: [`Playlist`]
    pub playlists: Option<Relationship<Playlist>>,
    /// The station associated with the artist. By default, station not included.
    ///
    /// Fetch limits: None (one station).
    ///
    /// Possible resources: [`Station`]
    pub station: Option<Relationship<Station>>,
}

/// Artist views
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(ArtistViewType, object = "artists", view)]
pub struct ArtistViews {
    /// A selection of albums from other artists this artist appears on
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "appears-on-albums")]
    pub appears_on_albums: Option<View<TitleOnlyAttribute, Album>>,
    /// Albums associated with the artist categorized as “compilations”
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "compilation-albums")]
    pub compilation_albums: Option<View<TitleOnlyAttribute, Album>>,
    /// A collection of albums selected as featured for the artist
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "featured-albums")]
    pub featured_albums: Option<View<TitleOnlyAttribute, Album>>,
    /// A collection of music videos selected as featured for the artist
    ///
    /// Possible resources: [`MusicVideo`]
    #[serde(rename = "featured-music-videos")]
    pub featured_music_videos: Option<View<TitleOnlyAttribute, MusicVideo>>,
    /// Relevant playlists associated with the artist
    ///
    /// Possible resources: [`Playlist`]
    #[serde(rename = "featured-playlists")]
    pub featured_playlists: Option<View<TitleOnlyAttribute, Playlist>>,
    /// Full-release albums associated with the artist
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "full-albums")]
    pub full_albums: Option<View<TitleOnlyAttribute, Album>>,
    /// The latest release for the artist deemed to still be recent
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "latest-release")]
    pub latest_release: Option<View<TitleOnlyAttribute, Album>>,
    /// Albums associated with the artist categorized as live performances
    ///
    /// Possible resources: [`Album`]
    #[serde(rename = "live-albums")]
    pub live_albums: Option<View<TitleOnlyAttribute, Album>>,
    /// Other artists similar to this artist
    ///
    /// Possible resources: [`Artist`]
    #[serde(rename = "similar-artists")]
    pub similar_artists: Option<View<TitleOnlyAttribute, Artist>>,
    /// Albums associated with the artist categorized as “singles”
    ///
    /// Possible resources: [`Album`]
    pub singles: Option<View<TitleOnlyAttribute, Album>>,
    /// Relevant music videos associated with the artist
    ///
    /// Possible resources: [`MusicVideo`]
    #[serde(rename = "top-music-videos")]
    pub top_music_videos: Option<View<TitleOnlyAttribute, MusicVideo>>,
    /// Songs associated with the artist based on popularity in the current storefront
    ///
    /// Possible resources: [`Song`]
    #[serde(rename = "top-songs")]
    pub top_songs: Option<View<TitleOnlyAttribute, Song>>,
}

/// Artist request builder
pub struct ArtistRequestBuilder;

/// Artist get request builder
pub type ArtistGetRequestBuilder<'a> = MusicRequestBuilder<'a, ArtistRequestBuilder>;

impl<'a> ArtistGetRequestBuilder<'a> {
    /// Fetch one artist by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Artist>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/artists/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple artists by id
    pub async fn many(mut self, client: &ApiClient, ids: &[&str]) -> Result<Vec<Artist>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/artists",
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
