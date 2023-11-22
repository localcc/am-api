//! History implementation

use crate::error::Error;
use crate::primitive::TrackType;
use crate::request::builder::MusicRequestBuilder;
use crate::request::paginated::paginate;
use crate::resource::Resource;
use crate::ApiClient;
use futures::Stream;

/// History
pub struct History;

impl History {
    /// Get history information
    pub fn get<'a>() -> HistoryGetRequestBuilder<'a> {
        HistoryGetRequestBuilder::default()
    }
}

/// History request builder
pub struct HistoryRequestBuilder;

/// History get request builder
pub type HistoryGetRequestBuilder<'a> = MusicRequestBuilder<'a, HistoryRequestBuilder>;

impl<'a> HistoryGetRequestBuilder<'a> {
    /// Fetch heavy rotation content
    ///
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    ///
    /// Possible resources: any
    pub async fn heavy_rotation(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<Resource, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/history/heavy-rotation"),
            request_context,
            offset,
        )
    }

    /// Fetch recently played resources
    ///
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    ///
    /// Possible resources: any
    pub async fn recently_played(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<Resource, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/recent/played"),
            request_context,
            offset,
        )
    }

    /// Fetch recently played tracks
    ///
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    ///
    /// Possible resources: [`LibraryMusicVideo`], [`LibrarySong`], [`MusicVideo`], [`Song`]
    pub async fn recently_played_tracks(
        mut self,
        client: &ApiClient,
        tracks: &[TrackType],
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<Resource, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));
        request_context.query.push((
            String::from("types"),
            tracks
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ));

        paginate(
            client.clone(),
            String::from("/v1/me/recent/played/tracks"),
            request_context,
            offset,
        )
    }

    /// Fetch recently played radio stations
    ///
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    ///
    /// Possible resources: [`Station`]
    pub async fn recently_played_stations(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<Resource, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/recent/radio-stations"),
            request_context,
            offset,
        )
    }

    /// Fetch resources recently added to the library
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    ///
    /// Possible resources: [`LibraryAlbum`], [`LibraryArtist`], [`LibraryPlaylist`], [`LibrarySong`]
    pub async fn recently_added_to_library(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<Resource, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/library/recently-added"),
            request_context,
            offset,
        )
    }
}
