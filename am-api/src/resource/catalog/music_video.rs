//! Music video

use crate::error::Error;
use crate::primitive::{ContentRating, EditorialNotes, PlayParameters, Preview};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::attributes::TitleOnlyAttribute;
use crate::resource::catalog::album::Album;
use crate::resource::catalog::artist::Artist;
use crate::resource::catalog::song::Song;
use crate::resource::genre::Genre;
use crate::resource::library::music_video::LibraryMusicVideo;
use crate::resource::relationship::Relationship;
use crate::resource::view::View;
use crate::resource::ResourceHeader;
use crate::time::year_or_date::YearOrDate;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Music video
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MusicVideo {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Music video attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<MusicVideoAttributes>,
    /// Music video relationships
    #[serde(default)]
    pub relationships: MusicVideoRelationships,
    /// The relationship views for the music video
    #[serde(default)]
    pub views: MusicVideoViews,
}

impl MusicVideo {
    /// Get music video request builder
    pub fn get<'a>() -> MusicVideoGetRequestBuilder<'a> {
        MusicVideoGetRequestBuilder::default()
    }
}

/// Music video attributes
#[derive(ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(
    MusicVideoAttributesExtension,
    object = "music-videos",
    extension,
    whitelist
)]
pub struct MusicVideoAttributes {
    /// The name of the album the music video appears on
    pub album_name: Option<String>,
    /// The artist’s name
    pub artist_name: String,
    /// **(Extended)** The URL of the artist for this content
    #[resource_property(whitelist, name = "artistUrl")]
    pub artist_url: Option<String>,
    /// The artwork for the music video’s associated album
    pub artwork: Option<Artwork>,
    /// The Recording Industry Association of America (RIAA) rating of the content. No value means no rating
    pub content_rating: Option<ContentRating>,
    /// Duration of the song in milliseconds
    pub duration_in_millis: u32,
    /// Editorial notes
    pub editorial_notes: Option<EditorialNotes>,
    /// Genre names
    pub genre_names: Vec<String>,
    /// Whether the music video has 4K content
    #[serde(rename = "has4K")]
    pub has_4k: bool,
    /// Whether the music video has HDR10-encoded content
    #[serde(rename = "hasHDR")]
    pub has_hdr: bool,
    /// The International Standard Recording Code (ISRC) for the music video
    pub isrc: Option<String>,
    /// The localized name of the music video
    pub name: String,
    /// The parameters to use to play back the music video
    pub play_params: Option<PlayParameters>,
    /// The preview assets for the music video
    pub previews: Vec<Preview>,
    /// The release date of the music video, when known, in YYYY-MM-DD or YYYY format. Prerelease music videos may have an expected release date in the future
    pub release_date: Option<YearOrDate>,
    /// The number of the music video in the album’s track list, when associated with an album
    pub track_number: Option<u32>,
    /// The URL for sharing the music video in Apple Music
    pub url: String,
    /// The video subtype associated with the content
    pub video_sub_type: Option<String>,
    /// (Classical music only) A unique identifier for the associated work
    pub work_id: Option<String>,
    /// (Classical music only) The name of the associated work
    pub work_name: Option<String>,
}

/// Music video relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(MusicVideoRelationshipType, object = "music-videos", relationship)]
pub struct MusicVideoRelationships {
    /// The albums associated with the music video. By default, albums includes identifiers only.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`Album`]
    pub albums: Option<Relationship<Album>>,
    /// The artists associated with the music video. By default, artists includes identifiers only.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`Artist`]
    pub artists: Option<Relationship<Artist>>,
    /// The genres associated with the music video. By default, genres not included.
    ///
    /// Fetch limits: None.
    ///
    /// Possible resources: [`Genre`]
    pub genres: Option<Relationship<Genre>>,
    /// The library of a music video if added to library.
    ///
    /// Fetch limits: None.
    ///
    /// Possible resources: [`LibraryMusicVideo`]
    pub library: Option<Relationship<LibraryMusicVideo>>,
    /// The songs associated with the music video.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`Song`]
    pub songs: Option<Relationship<Song>>,
}

/// Music video views
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(MusicVideoViewType, object = "music-videos", view)]
pub struct MusicVideoViews {
    /// More music videos of some type by the artist.
    ///
    /// Fetch limits: 15 default, 100 maximum.
    ///
    /// Possible resources: [`MusicVideo`]
    #[serde(rename = "more-by-artist")]
    pub more_by_artist: Option<View<TitleOnlyAttribute, MusicVideo>>,
    /// More music videos in the given music video genre.
    ///
    /// Fetch limits: 15 default, 100 maximum.
    ///
    /// Possible resources: [`MusicVideo`]
    #[serde(rename = "more-in-genre")]
    pub more_in_genre: Option<View<TitleOnlyAttribute, MusicVideo>>,
}

/// Music video request builder
pub struct MusicVideoRequestBuilder;

/// Music video get request builder
pub type MusicVideoGetRequestBuilder<'a> = MusicRequestBuilder<'a, MusicVideoRequestBuilder>;

impl<'a> MusicVideoGetRequestBuilder<'a> {
    /// Fetch one music video by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<MusicVideo>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/music-videos/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple music videos by id
    ///
    /// # Params
    ///
    /// * isrc - if the ids are ISRCs or music video ids, false means music video ids, true means ISRCs
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
        isrc: bool,
    ) -> Result<Vec<MusicVideo>, Error> {
        let mut request_context = self.get_request_context_drain(client);

        let id_query = match isrc {
            true => "filter[isrc]",
            false => "ids",
        };
        request_context
            .query
            .push((id_query.to_string(), ids.to_vec().join(",")));

        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/music-videos",
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
