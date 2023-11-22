//! Library album

use crate::error::Error;
use crate::primitive::{ContentRating, PlayParameters};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::catalog::album::Album;
use crate::resource::library::artist::LibraryArtist;
use crate::resource::relationship::Relationship;
use crate::resource::{Resource, ResourceHeader};
use crate::time::year_or_date::YearOrDate;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;


/// Library album
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LibraryAlbum {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Album attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<LibraryAlbumAttributes>,
    /// Album relationships
    #[serde(default)]
    pub relationships: LibraryAlbumRelationships,
}

/// Library album attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct LibraryAlbumAttributes {
    /// The artist's name
    pub artist_name: String,
    /// The album artwork
    pub artwork: Artwork,
    /// The Recording Industry Association of America (RIAA) rating of the content. The possible values for this rating are clean and explicit. No value means no rating
    pub content_rating: Option<ContentRating>,
    /// The date the album was added to the library, in YYYY-MM-DD or YYYY format
    pub date_added: Option<YearOrDate>,
    /// The localized name of the album
    pub name: String,
    /// The parameters to use to playback the tracks of the album
    pub play_params: Option<PlayParameters>,
    /// The release date of the album, when known, in YYYY-MM-DD or YYYY format. Pre-release albums may have an expected release date in the future
    pub release_date: Option<YearOrDate>,
    /// The number of tracks
    pub track_count: u32,
    /// The names of the genres associated with this album
    pub genre_names: Vec<String>,
}

/// Library album relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(LibraryAlbumRelationshipType, object = "library-albums", relationship)]
pub struct LibraryAlbumRelationships {
    /// The library artists associated with the album. By default, artists not included.
    ///
    /// Fetch limits: 10 default, 10 maximum
    ///
    /// Possible resources: [`LibraryArtist`]
    pub artists: Option<Relationship<LibraryArtist>>,
    /// The album in the Apple Music catalog the library album is associated with, when known.
    ///
    /// Fetch limits: None (associated with at most one catalog album)
    ///
    /// Possible resources: [`Album`]
    pub catalog: Option<Relationship<Album>>,
    /// The library songs and library music videos on the album. Only available when fetching single library album resource by ID. By default, tracks includes objects.
    ///
    /// Fetch limits: 300 default, 300 maximum
    ///
    /// Possible resources: [`LibrarySong`], [`LibraryMusicVideo`]
    pub tracks: Option<Relationship<Resource>>,
}

/// Library album request builder
pub struct LibraryAlbumRequestBuilder;

/// Library album get request builder
pub type LibraryAlbumGetRequestBuilder<'a> = MusicRequestBuilder<'a, LibraryAlbumRequestBuilder>;

impl<'a> LibraryAlbumGetRequestBuilder<'a> {
    /// Fetch one library album by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<LibraryAlbum>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!("/v1/me/library/albums/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple library albums by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<LibraryAlbum>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/me/library/albums")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch all library albums
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
    ) -> impl Stream<Item = Result<LibraryAlbum, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/library/albums"),
            request_context,
            offset,
        )
    }
}
