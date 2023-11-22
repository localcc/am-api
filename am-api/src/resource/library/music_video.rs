//! Library music video

use crate::error::Error;
use crate::primitive::{ContentRating, PlayParameters};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::catalog::music_video::MusicVideo;
use crate::resource::library::album::LibraryAlbum;
use crate::resource::library::artist::LibraryArtist;
use crate::resource::relationship::Relationship;
use crate::resource::ResourceHeader;
use crate::time::year_or_date::YearOrDate;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Library music video
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LibraryMusicVideo {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Library music video attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<LibraryMusicVideoAttributes>,
    /// Library music video relationships
    #[serde(default)]
    pub relationships: LibraryMusicVideoRelationships,
}

/// Library music video attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct LibraryMusicVideoAttributes {
    /// The name of the album the music video appears on
    pub album_name: Option<String>,
    /// The artist’s name
    pub artist_name: String,
    /// The artwork for the music video’s associated album
    pub artwork: Artwork,
    /// The Recording Industry Association of America (RIAA) rating of the content. The possible values for this rating are clean and explicit. No value means no rating
    pub content_rating: Option<ContentRating>,
    /// The duration of the music video in milliseconds
    pub duration_in_millis: u32,
    /// The names of the genres associated with this music video
    pub genre_names: Vec<String>,
    /// The localized name of the music video
    pub name: String,
    /// The parameters to use to playback the music video
    pub play_params: Option<PlayParameters>,
    /// The release date of the music video, when known, in YYYY-MM-DD or YYYY format. Pre-release content may have an expected release date in the future
    pub release_date: Option<YearOrDate>,
    /// The number of the music video in the album’s track list
    pub track_number: Option<u32>,
}

/// Library music video relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(
    LibraryMusicVideoRelationshipType,
    object = "library-music-videos",
    relationship
)]
pub struct LibraryMusicVideoRelationships {
    /// The library albums associated with the music video. By default, albums not included.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`LibraryAlbum`]
    pub albums: Option<Relationship<LibraryAlbum>>,
    /// The library artists associated with the music video. By default, artists not included.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`LibraryArtist`]
    pub artists: Option<Relationship<LibraryArtist>>,
    /// The music video in the Apple Music catalog the library music video is associated with, when known.
    ///
    /// Fetch limits: None (associated with at most one catalog music video).
    ///
    /// Possible resources: [`MusicVideo`]
    pub catalog: Option<Relationship<MusicVideo>>,
}

/// Library music video request builder
pub struct LibraryMusicVideoRequestBuilder;

/// Library music video get request builder
pub type LibraryMusicVideoGetRequestBuilder<'a> =
    MusicRequestBuilder<'a, LibraryMusicVideoRequestBuilder>;

impl<'a> LibraryMusicVideoGetRequestBuilder<'a> {
    /// Fetch one library music video by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<LibraryMusicVideo>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!("/v1/me/library/music-videos/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple library music videos by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<LibraryMusicVideo>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/me/library/music-videos")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch all library music videos
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    pub fn all(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<LibraryMusicVideo, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/library/music-videos"),
            request_context,
            offset,
        )
    }
}
