//! Catalog search

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::resource::catalog::activity::Activity;
use crate::resource::catalog::album::Album;
use crate::resource::catalog::artist::Artist;
use crate::resource::catalog::curator::{AppleCurator, Curator};
use crate::resource::catalog::music_video::MusicVideo;
use crate::resource::catalog::playlist::Playlist;
use crate::resource::catalog::record_label::RecordLabel;
use crate::resource::catalog::song::Song;
use crate::resource::catalog::station::Station;
use crate::resource::search::SearchRelationship;
use crate::resource::ErrorResponse;
use crate::ApiClient;
use am_api_proc_macro::Context;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// Catalog search
pub struct CatalogSearch;

impl CatalogSearch {
    /// Catalog search
    pub fn search<'a>() -> CatalogSearchRequestBuilder<'a> {
        CatalogSearchRequestBuilder::default()
    }
}

/// Catalog search request builder marker
pub struct CatalogSearchRequestBuilderMarker;

/// Catalog search request builder
pub type CatalogSearchRequestBuilder<'a> =
    MusicRequestBuilder<'a, CatalogSearchRequestBuilderMarker>;

impl<'a> CatalogSearchRequestBuilder<'a> {
    /// Search the catalog using a query
    ///
    /// # Params
    ///
    /// * types - types to include in the search results
    ///
    /// * term - The entered text for the search
    pub async fn search(
        mut self,
        client: &ApiClient,
        types: &[CatalogSearchType],
        term: &str,
    ) -> Result<CatalogSearchResults, Error> {
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
            .push((String::from("term"), term.to_string()));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/search",
                storefront = request_context.storefront.alpha2
            ))
            .query(&request_context.query)
            .send()
            .await?;

        request_context.query.pop();
        request_context.query.pop();

        let request_context = Arc::new(request_context);

        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        let mut response = response
            .json::<CatalogSearchResponse<CatalogSearchResults>>()
            .await?;
        response.results.set_context(request_context);
        Ok(response.results)
    }

    /// Get catalog search hints
    ///
    /// # Params
    ///
    /// * term - The entered text for the search
    ///
    /// * limit - returned hints count limit
    pub async fn search_hints(
        mut self,
        client: &ApiClient,
        term: &str,
        limit: usize,
    ) -> Result<Vec<String>, Error> {
        let mut request_context = self.get_request_context_drain(client);

        request_context
            .query
            .push((String::from("limit"), limit.to_string()));
        request_context
            .query
            .push((String::from("term"), term.to_string()));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/search/hints",
                storefront = request_context.storefront.alpha2
            ))
            .query(&request_context.query)
            .send()
            .await?;

        request_context.query.pop();

        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        let response: CatalogSearchResponse<CatalogSearchHints> = response.json().await?;
        Ok(response.results.terms)
    }

    /// Get catalog search suggestions
    /// # Params
    ///
    /// * term - The entered text for the search
    ///
    /// * types - types to include in the search results
    ///
    /// * limit - returned hints count limit **(Default value: 5, maximum: 10)**
    pub async fn suggestions(
        mut self,
        client: &ApiClient,
        kinds: &[SuggestionKind],
        types: &[CatalogSearchType],
        term: &str,
        limit: usize,
    ) -> Result<Vec<CatalogSearchSuggestion>, Error> {
        let mut request_context = self.get_request_context_drain(client);

        request_context
            .query
            .push((String::from("limit"), limit.to_string()));
        request_context.query.push((
            String::from("types"),
            types
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ));
        request_context.query.push((
            String::from("kinds"),
            kinds
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ));
        request_context
            .query
            .push((String::from("term"), term.to_string()));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/search/suggestions",
                storefront = request_context.storefront.alpha2
            ))
            .query(&request_context.query)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        let response = response
            .json::<CatalogSearchResponse<CatalogSearchSuggestions>>()
            .await?;
        Ok(response.results.suggestions)
    }
}

/// Catalog search results
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(default)]
pub struct CatalogSearchResults {
    /// Activities
    pub activities: SearchRelationship<Activity>,
    /// Albums
    pub albums: SearchRelationship<Album>,
    /// Apple curators
    #[serde(rename = "apple-curators")]
    pub apple_curators: SearchRelationship<AppleCurator>,
    /// Curators
    pub curators: SearchRelationship<Curator>,
    /// Artists
    pub artists: SearchRelationship<Artist>,
    /// Music videos
    #[serde(rename = "music-videos")]
    pub music_videos: SearchRelationship<MusicVideo>,
    /// Playlists
    pub playlists: SearchRelationship<Playlist>,
    /// Record labels
    #[serde(rename = "record-labels")]
    pub record_labels: SearchRelationship<RecordLabel>,
    /// Songs
    pub songs: SearchRelationship<Song>,
    /// Stations
    pub stations: SearchRelationship<Station>,
}

/// Catalog search type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CatalogSearchType {
    /// Activities
    Activities,
    /// Albums
    Albums,
    /// Apple curators
    AppleCurators,
    /// Curators
    Curators,
    /// Artists
    Artists,
    /// Music videos
    MusicVideos,
    /// Playlists
    Playlists,
    /// Record labels
    RecordLabels,
    /// Songs
    Songs,
    /// Stations
    Stations,
}

impl Display for CatalogSearchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CatalogSearchType::Activities => "activities",
            CatalogSearchType::Albums => "albums",
            CatalogSearchType::AppleCurators => "apple-curators",
            CatalogSearchType::Curators => "curators",
            CatalogSearchType::Artists => "artists",
            CatalogSearchType::MusicVideos => "music-videos",
            CatalogSearchType::Playlists => "playlists",
            CatalogSearchType::RecordLabels => "record-labels",
            CatalogSearchType::Songs => "songs",
            CatalogSearchType::Stations => "stations",
        };
        write!(f, "{}", s)
    }
}

/// Catalog search hints
#[derive(Serialize, Deserialize)]
struct CatalogSearchHints {
    /// Search hints
    terms: Vec<String>,
}

/// Catalog search suggestion kind
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum SuggestionKind {
    /// Terms
    Terms,
    /// Top results
    TopResults,
}

impl Display for SuggestionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SuggestionKind::Terms => "terms",
            SuggestionKind::TopResults => "topResults",
        };
        write!(f, "{}", s)
    }
}

/// Catalog search suggestion
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CatalogSearchSuggestion {
    /// Suggestion kind
    pub kind: SuggestionKind,
    /// Search term
    pub search_term: String,
    /// Display term
    pub display_term: String,
}

/// Catalog search suggestions
#[derive(Serialize, Deserialize)]
struct CatalogSearchSuggestions {
    /// Suggestions
    suggestions: Vec<CatalogSearchSuggestion>,
}

/// Catalog search response
#[derive(Serialize, Deserialize)]
struct CatalogSearchResponse<T> {
    /// Results
    pub results: T,
}
