//! Station

use crate::error::Error;
use crate::primitive::{ContentRating, EditorialNotes, PlayParameters};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::catalog::curator::AppleCurator;
use crate::resource::relationship::Relationship;
use crate::resource::ResourceHeader;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Station
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Station attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<StationAttributes>,
    /// Station relationships
    #[serde(default)]
    pub relationships: StationRelationships,
}

impl Station {
    /// Get station request builder
    pub fn get<'a>() -> StationGetRequestBuilder<'a> {
        StationGetRequestBuilder::default()
    }
}

/// Station attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct StationAttributes {
    /// The radio station artwork
    pub artwork: Artwork,
    /// The duration of the stream. This value isn’t emitted for ‘live’ or programmed stations
    pub duration_in_millis: Option<u32>,
    /// The notes about the station that appear in Apple Music
    pub editorial_notes: Option<EditorialNotes>,
    /// The episode number of the station. This value appears when the station represents an episode of a show or other content
    pub episode_number: Option<String>,
    /// The rating of the content possibly heard while playing the station. The possible values for this rating are clean and explicit. No value means no rating
    pub content_rating: Option<ContentRating>,
    /// Whether the station is a live stream
    pub is_live: bool,
    /// The media kind for the station. It can have value audio or video depending on whether it has video stream or audio stream
    pub media_kind: MediaKind,
    /// The localized name of the station
    pub name: String,
    /// The parameters to use to play back the station
    pub play_params: Option<PlayParameters>,
    /// The name of the entity that provided the station, when specified
    pub station_provider_name: Option<String>,
    /// The URL for sharing the station in Apple Music
    pub url: String,
}

/// Station relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(StationRelationshipType, object = "stations", relationship)]
pub struct StationRelationships {
    /// For radio show episodes, this relationship is the Apple Curator that represents the radio show
    ///
    /// Possible resources: [`AppleCurator`]
    #[serde(rename = "radio-show")]
    pub radio_show: Option<Relationship<AppleCurator>>,
}

/// Station request builder
pub struct StationRequestBuilder;

/// Station get request builder
pub type StationGetRequestBuilder<'a> = MusicRequestBuilder<'a, StationRequestBuilder>;

impl<'a> StationGetRequestBuilder<'a> {
    /// Fetch one station by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Station>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/stations/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple stations by id
    pub async fn many(mut self, client: &ApiClient, ids: &[&str]) -> Result<Vec<Station>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/stations",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch live radio stations
    pub async fn live(mut self, client: &ApiClient) -> Result<Vec<Station>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context.query.push((
            String::from("filter[featured]"),
            String::from("apple-music-live-radio"),
        ));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/stations",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch user's current personal station
    pub async fn personal(mut self, client: &ApiClient) -> Result<Option<Station>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("filter[identity]"), String::from("personal")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/stations",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }
}

/// Media kind
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaKind {
    /// Audio
    #[serde(rename = "audio")]
    #[default]
    Audio,
    /// Video
    #[serde(rename = "video")]
    Video,
}

/// Station genre
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct StationGenre {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Station genre attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<StationGenreAttributes>,
    /// Station genre relationships
    #[serde(default)]
    pub relationships: StationGenreRelationships,
}

impl StationGenre {
    /// Get station genre request builder
    pub fn get<'a>() -> StationGenreGetRequestBuilder<'a> {
        StationGenreGetRequestBuilder::default()
    }
}

/// Station genre attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct StationGenreAttributes {
    /// The name of the station genre
    pub name: String,
}

/// Station genre relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(StationGenreRelationshipType, object = "station-genres", relationship)]
pub struct StationGenreRelationships {
    /// Stations associated with the station genre
    ///
    /// Possible resources: [`Station`]
    pub stations: Option<Relationship<Station>>,
}

/// Station genre request builder
pub struct StationGenreRequestBuilder;

/// Station genre get request builder
pub type StationGenreGetRequestBuilder<'a> = MusicRequestBuilder<'a, StationGenreRequestBuilder>;

impl<'a> StationGenreGetRequestBuilder<'a> {
    /// Fetch one station genre by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<StationGenre>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/station-genres/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple station genres by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<StationGenre>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/station-genres",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch all station genres
    ///
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
    ) -> impl Stream<Item = Result<StationGenre, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            format!(
                "/v1/catalog/{storefront}/station-genres",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ),
            request_context,
            offset,
        )
    }
}
