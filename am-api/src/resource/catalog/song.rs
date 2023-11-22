//! Song

use crate::error::Error;
use crate::primitive::{AudioVariant, ContentRating, EditorialNotes, PlayParameters, Preview};
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::catalog::album::Album;
use crate::resource::catalog::artist::Artist;
use crate::resource::catalog::music_video::MusicVideo;
use crate::resource::catalog::station::Station;
use crate::resource::genre::Genre;
use crate::resource::library::song::LibrarySong;
use crate::resource::relationship::Relationship;
use crate::resource::ResourceHeader;
use crate::time::year_or_date::YearOrDate;
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Song
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Song attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<SongAttributes>,
    /// Song relationships
    #[serde(default)]
    pub relationships: SongRelationships,
}

impl Song {
    /// Get song request builder
    pub fn get<'a>() -> SongGetRequestBuilder<'a> {
        SongGetRequestBuilder::default()
    }
}

/// Song attributes
#[derive(ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(SongAttributesExtension, object = "songs", extension, whitelist)]
pub struct SongAttributes {
    /// Song album name
    pub album_name: String,
    /// Song artist name
    pub artist_name: String,
    /// **(Extended)** Song artist url
    #[resource_property(whitelist, name = "artistUrl")]
    pub artist_url: Option<String>,
    /// Album artwork
    pub artwork: Artwork,
    /// (Classical music only) Name of the artist or composer to attribute the song with
    pub attribution: Option<String>,
    /// **(Extended)** Specific audio variants for a song
    #[resource_property(whitelist, name = "audioVariants")]
    pub audio_variants: Option<Vec<AudioVariant>>,
    /// Song composer
    pub composer: Option<String>,
    /// The Recording Industry Association of America (RIAA) rating of the content. No value means no rating
    pub content_rating: Option<ContentRating>,
    /// Disc number of the album this song appears on
    pub disc_number: Option<u32>,
    /// Duration of the song in milliseconds
    pub duration_in_millis: u32,
    /// Editorial notes
    pub editorial_notes: Option<EditorialNotes>,
    /// Genre names
    pub genre_names: Vec<String>,
    /// Indicates whether the song has lyrics available in the Apple Music catalog
    pub has_lyrics: bool,
    /// Indicates whether the response delivered the song as an Apple Digital Master
    pub is_apple_digital_master: bool,
    /// The International Standard Recording Code (ISRC) for the song
    pub isrc: Option<String>,
    /// (Classical music only) The movement count of the song
    pub movement_count: u32,
    /// (Classical music only) The movement name of the song
    pub movement_name: String,
    /// (Classical music only) The movement number of the song
    pub movement_number: u32,
    /// The localized name of the song
    pub name: String,
    /// The parameters to use to play back the song
    pub play_params: Option<PlayParameters>,
    /// The preview assets for the song
    pub previews: Vec<Preview>,
    /// The release date of the song, when known, in YYYY-MM-DD or YYYY format. Prerelease songs may have an expected release date in the future
    pub release_date: Option<YearOrDate>,
    /// The number of the song in the album's track list
    pub track_number: u32,
    /// (Required) The URL for sharing the song in Apple Music
    pub url: String,
    /// (Classical music only) The name of the associated work
    pub work_name: Option<String>,
}

/// Song relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(SongRelationshipType, object = "songs", relationship)]
pub struct SongRelationships {
    /// The albums associated with the song. By default, albums includes identifiers only.
    ///
    /// Fetch limits: 10 default, 10 maximum
    ///
    /// Possible resources: [`Album`]
    pub albums: Option<Relationship<Album>>,
    /// The artists associated with the song. By default, artists includes identifiers only.
    ///
    /// Fetch limits: 10 default, 10 maximum
    ///
    /// Possible resources: [`Artist`]
    pub artists: Option<Relationship<Artist>>,
    /// The composers for a catalog song
    ///
    /// Possible resources: [`Artist`]
    pub composers: Option<Relationship<Artist>>,
    /// The genres associated with the song. By default, genres is not included.
    ///
    /// Fetch limits: None
    ///
    /// Possible resources: [`Genre`]
    pub genres: Option<Relationship<Genre>>,
    /// Library song for a catalog song if added to library
    ///
    /// Possible resources: [`LibrarySong`]
    pub library: Option<Relationship<LibrarySong>>,
    /// Music videos for a catalog song
    ///
    /// Possible resources: [`MusicVideo`]
    #[serde(rename = "music-videos")]
    pub music_videos: Option<Relationship<MusicVideo>>,
    /// The station associated with the song. By default, station is not included.
    ///
    /// Fetch limits: None
    ///
    /// Possible resources: [`Station`]
    pub station: Option<Relationship<Station>>,
}

/// Song request builder
pub struct SongRequestBuilder;

/// Song get request builder
pub type SongGetRequestBuilder<'a> = MusicRequestBuilder<'a, SongRequestBuilder>;

impl<'a> SongGetRequestBuilder<'a> {
    /// Fetch one song by id
    pub async fn one(mut self, client: &ApiClient, id: &str) -> Result<Option<Song>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));
        let response = client
            .get(&format!(
                "/v1/catalog/{storefront}/songs/{id}",
                storefront = request_context.storefront.alpha2.to_lowercase()
            ))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple songs by id
    ///
    /// # Params
    ///
    /// * isrc - if the ids are ISRCs or song ids, false means song ids, true means ISRCs
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
        isrc: bool,
    ) -> Result<Vec<Song>, Error> {
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
                "/v1/catalog/{storefront}/songs",
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
