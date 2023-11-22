//! Library search

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::resource::library::album::LibraryAlbum;
use crate::resource::library::artist::LibraryArtist;
use crate::resource::library::music_video::LibraryMusicVideo;
use crate::resource::library::playlist::LibraryPlaylist;
use crate::resource::library::song::LibrarySong;
use crate::resource::relationship::Relationship;
use crate::resource::ErrorResponse;
use crate::ApiClient;
use am_api_proc_macro::Context;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// Library search
pub struct LibrarySearch;

/// Library search request builder marker
pub struct LibrarySearchRequestBuilderMarker;

/// Library search request builder
pub type LibrarySearchRequestBuilder<'a> =
    MusicRequestBuilder<'a, LibrarySearchRequestBuilderMarker>;

impl<'a> LibrarySearchRequestBuilder<'a> {
    /// Search the library using a query
    ///
    /// # Params
    ///
    /// * types - types to search
    ///
    /// * term - The entered text for the search, spaces will automatically get replaced with '+'
    pub async fn search(
        mut self,
        client: &ApiClient,
        types: &[LibrarySearchType],
        term: &str,
    ) -> Result<LibrarySearchResults, Error> {
        let mut request_context = self.get_request_context_drain(client);

        request_context.query.push((
            String::from("types"),
            types
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ));
        request_context
            .query
            .push((String::from("term"), term.to_string().replace(' ', "+")));

        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/me/library/search")
            .query(&request_context.query)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        let mut response = response.json::<LibrarySearchResponse>().await?;
        response.results.set_context(request_context);
        Ok(response.results)
    }
}

/// Library search results
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(default)]
pub struct LibrarySearchResults {
    /// Library albums
    #[serde(rename = "library-albums")]
    pub library_albums: Relationship<LibraryAlbum>,
    /// Library artists
    #[serde(rename = "library-artists")]
    pub library_artists: Relationship<LibraryArtist>,
    /// Library music videos
    #[serde(rename = "library-music-videos")]
    pub library_music_videos: Relationship<LibraryMusicVideo>,
    /// Library playlists
    #[serde(rename = "library-playlists")]
    pub library_playlists: Relationship<LibraryPlaylist>,
    /// Library songs
    #[serde(rename = "library-songs")]
    pub library_songs: Relationship<LibrarySong>,
}

/// Library search type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibrarySearchType {
    /// Library albums
    LibraryAlbums,
    /// Library artists
    LibraryArtists,
    /// Library music videos
    LibraryMusicVideos,
    /// Library playlists
    LibraryPlaylists,
    /// Library songs
    LibrarySongs,
}

impl Display for LibrarySearchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LibrarySearchType::LibraryAlbums => "library-albums",
            LibrarySearchType::LibraryArtists => "library-artists",
            LibrarySearchType::LibraryMusicVideos => "library-music-videos",
            LibrarySearchType::LibraryPlaylists => "library-playlists",
            LibrarySearchType::LibrarySongs => "library-songs",
        };
        write!(f, "{}", s)
    }
}

/// Library search response
#[derive(Serialize, Deserialize)]
struct LibrarySearchResponse {
    /// Results
    results: LibrarySearchResults,
}
