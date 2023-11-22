//! Library song

use crate::error::Error;
use crate::primitive::{ContentRating, PlayParameters};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::catalog::song::Song;
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

/// Library song
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LibrarySong {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Library song attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<LibrarySongAttributes>,
    /// Library song relationships
    #[serde(default)]
    pub relationships: LibrarySongRelationships,
}

/// Library song attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct LibrarySongAttributes {
    /// The name of the album the song appears on
    pub album_name: Option<String>,
    /// The artist’s name
    pub artist_name: String,
    /// The album artwork
    pub artwork: Artwork,
    /// The Recording Industry Association of America (RIAA) rating of the content. The possible values for this rating are clean and explicit. No value means no rating
    pub content_rating: Option<ContentRating>,
    /// The disc number the song appears on
    pub disc_number: Option<u32>,
    /// The approximate length of the song in milliseconds
    pub duration_in_millis: u32,
    /// The genre names the song is associated with
    pub genre_names: Vec<String>,
    /// Indicates if the song has lyrics available in the Apple Music catalog. If true, the song has lyrics available; otherwise, it does not
    pub has_lyrics: bool,
    /// The localized name of the song
    pub name: String,
    /// The parameters to use to playback the song
    pub play_params: Option<PlayParameters>,
    /// The release date of the song, when known, in YYYY-MM-DD or YYYY format. Pre-release songs may have an expected release date in the future
    pub release_date: Option<YearOrDate>,
    /// The number of the song in the album’s track list
    pub track_number: Option<u32>,
}

/// Library song relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(LibrarySongRelationshipType, object = "library-songs", relationship)]
pub struct LibrarySongRelationships {
    /// The library albums associated with the song. By default, albums not included.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`LibraryAlbum`]
    pub albums: Option<Relationship<LibraryAlbum>>,
    /// The library artists associated with the song. By default, artists not included.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: [`LibraryArtist`]
    pub artists: Option<Relationship<LibraryArtist>>,
    /// The song in the Apple Music catalog the library song is associated with, when known.
    ///
    /// Fetch limits: None.
    ///
    /// Possible resources: [`Song`]
    pub catalog: Option<Relationship<Song>>,
}

/// Library song request builder
pub struct LibrarySongRequestBuilder;

/// Library song get request builder
pub type LibrarySongGetRequestBuilder<'a> = MusicRequestBuilder<'a, LibrarySongRequestBuilder>;

impl<'a> LibrarySongGetRequestBuilder<'a> {
    /// Fetch one library song by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<LibrarySong>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!("/v1/me/library/songs/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple library songs by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<LibrarySong>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/me/library/songs")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch all library songs
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
    ) -> impl Stream<Item = Result<LibrarySong, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/library/songs"),
            request_context,
            offset,
        )
    }
}
