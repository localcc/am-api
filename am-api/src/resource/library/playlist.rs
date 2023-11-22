//! Library playlist

use crate::error::Error;
use crate::primitive::PlayParameters;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::artwork::Artwork;
use crate::resource::attributes::DescriptionAttribute;
use crate::resource::catalog::playlist::Playlist;
use crate::resource::relationship::Relationship;
use crate::resource::{ErrorResponse, Resource, ResourceHeader, ResourceInfo, ResourceType};
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use futures::Stream;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;

/// Library playlist
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LibraryPlaylist {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Library playlist attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<LibraryPlaylistAttributes>,
    /// Library playlist relationships
    #[serde(default)]
    pub relationships: LibraryPlaylistRelationships,
}

impl LibraryPlaylist {
    /// Get library playlist request builder
    pub fn get<'a>() -> LibraryPlaylistGetRequestBuilder<'a> {
        LibraryPlaylistGetRequestBuilder::default()
    }

    /// Create a new library playlist
    pub fn create<'localization>(name: &str) -> LibraryPlaylistCreateBuilder<'localization, '_> {
        LibraryPlaylistCreateBuilder::new(name)
    }

    /// Add tracks to this library playlist
    pub async fn add_tracks(&self, client: &ApiClient, tracks: &[&Resource]) -> Result<(), Error> {
        let mut request = LibraryPlaylistAddRequest::default();

        for track in tracks {
            let supported = matches!(
                track,
                Resource::MusicVideo { .. }
                    | Resource::Song { .. }
                    | Resource::LibraryMusicVideo { .. }
                    | Resource::LibrarySong { .. }
            );

            if !supported {
                return Err(Error::InvalidResourceType);
            }

            request.data.push(ThinRelationship {
                id: track.get_header().id.clone(),
                ty: track.get_type().to_string(),
            });
        }

        let response = client
            .post(&format!(
                "/v1/me/library/playlists/{}/tracks",
                self.header.id
            ))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_response: ErrorResponse = response.json().await?;
            return Err(Error::MusicError(error_response));
        }

        Ok(())
    }
}

/// Library playlist attributes
#[derive(ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(
    LibraryPlaylistAttributesExtension,
    object = "library-playlists",
    extension,
    whitelist
)]
pub struct LibraryPlaylistAttributes {
    /// Playlist artwork
    pub artwork: Option<Artwork>,
    /// Indicates whether the playlist is editable
    pub can_edit: bool,
    /// The date and time the playlist was added to the user’s library.
    /// In YYYY-MM-DDThh:mm:ssZ ISO 8601 format
    #[serde(with = "time::serde::iso8601::option")]
    pub date_added: Option<OffsetDateTime>,
    /// A description of the playlist
    pub description: Option<DescriptionAttribute>,
    /// Indicates whether the playlist has a representation in the Apple Music catalog
    pub has_catalog: bool,
    /// The localized name of the playlist
    pub name: String,
    /// The parameters to use to play back the tracks in the playlist
    pub play_params: Option<PlayParameters>,
    /// A flag to indicate whether the library playlist is a public playlist
    pub is_public: bool,
    /// **(Extended)** The resource types that are present in the tracks of the library playlist
    #[resource_property(whitelist, name = "trackTypes")]
    pub track_types: Option<Vec<LibraryTrackTypes>>,
}

/// Library playlist relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(
    LibraryPlaylistRelationshipType,
    object = "library-playlists",
    relationship
)]
pub struct LibraryPlaylistRelationships {
    /// The corresponding playlist in the Apple Music catalog the playlist is associated with.
    ///
    /// Fetch limits: None (associated with at most one catalog playlist)
    ///
    /// Possible resources: [`Playlist`]
    pub catalog: Option<Relationship<Playlist>>,
    /// The library songs and library music videos included in the playlist. By default, tracks not included. Only available when fetching a single library playlist resource by ID.
    ///
    /// Fetch limits: 100 default, 100 maximum.
    ///
    /// Possible resources: [`LibrarySong`], [`LibraryMusicVideo`]
    pub tracks: Option<Relationship<Resource>>,
}

/// Library playlist request builder
pub struct LibraryPlaylistRequestBuilder;

/// Library playlist get request builder
pub type LibraryPlaylistGetRequestBuilder<'a> =
    MusicRequestBuilder<'a, LibraryPlaylistRequestBuilder>;

impl<'a> LibraryPlaylistGetRequestBuilder<'a> {
    /// Fetch one library playlist by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<LibraryPlaylist>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!("/v1/me/library/playlists/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple library playlists by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<LibraryPlaylist>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/me/library/playlists")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch all library playlists
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
    ) -> impl Stream<Item = Result<LibraryPlaylist, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/library/playlists"),
            request_context,
            offset,
        )
    }
}

/// Library playlist creation request
#[derive(Serialize, Clone, PartialEq, Eq)]
pub struct LibraryPlaylistCreateRequest<'a> {
    /// Creation attributes
    attributes: LibraryPlaylistCreateRequestAttributes<'a>,
    /// Relationships
    relationships: LibraryPlaylistCreateRequestRelationships,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct LibraryPlaylistCreateRequestAttributes<'a> {
    /// Playlist name
    name: &'a str,
    /// Playlist description
    description: Option<String>,
    /// Is playlist public
    #[serde(rename = "isPublic")]
    public: bool,
}

#[derive(Serialize, Default, Clone, PartialEq, Eq)]
struct LibraryPlaylistCreateRequestRelationships {
    #[serde(skip_serializing_if = "Option::is_none")]
    tracks: Option<AddedTrackRelationships>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<ParentRelationship>,
}

#[derive(Serialize, Default, Clone, PartialEq, Eq)]
struct AddedTrackRelationships {
    data: Vec<ThinRelationship>,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct ParentRelationship {
    data: [ThinRelationship; 1],
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct ThinRelationship {
    /// Relationship id
    id: String,
    /// Relationship type
    #[serde(rename = "type")]
    ty: String,
}

/// Library playlist create request builder
pub type LibraryPlaylistCreateBuilder<'localization, 'name> = MusicRequestBuilder<
    'localization,
    LibraryPlaylistRequestBuilder,
    LibraryPlaylistCreateRequest<'name>,
>;

impl<'localization, 'name> LibraryPlaylistCreateBuilder<'localization, 'name> {
    /// Create a new [`LibraryPlaylistCreateBuilder`] instance
    pub fn new(name: &'name str) -> Self {
        LibraryPlaylistCreateBuilder {
            storefront_override: None,
            localization_override: None,
            extensions: Default::default(),
            relationships: Default::default(),
            views: Default::default(),
            data: LibraryPlaylistCreateRequest {
                attributes: LibraryPlaylistCreateRequestAttributes {
                    name,
                    description: None,
                    public: false,
                },
                relationships: Default::default(),
            },
            _marker: Default::default(),
        }
    }

    /// Set description for this request
    pub fn description(mut self, desc: &str) -> Self {
        self.data.attributes.description = Some(desc.to_string());
        self
    }

    /// Set if the playlist is public for this request
    pub fn public(mut self, p: bool) -> Self {
        self.data.attributes.public = p;
        self
    }

    /// Add tracks to this request
    pub fn tracks(mut self, resources: &[&Resource]) -> Result<Self, Error> {
        let tracks = self
            .data
            .relationships
            .tracks
            .get_or_insert_with(AddedTrackRelationships::default);

        for resource in resources {
            let supported = matches!(
                resource,
                Resource::MusicVideo { .. }
                    | Resource::Song { .. }
                    | Resource::LibraryMusicVideo { .. }
                    | Resource::LibrarySong { .. }
            );

            if !supported {
                return Err(Error::InvalidResourceType);
            }

            let ty = resource.get_type();

            tracks.data.push(ThinRelationship {
                id: resource.get_header().id.clone(),
                ty: ty.to_string(),
            });
        }

        Ok(self)
    }

    /// Set parent playlist folder
    pub fn parent_folder(mut self, parent_folder_id: &str) -> Self {
        self.data.relationships.parent = Some(ParentRelationship {
            data: [ThinRelationship {
                id: parent_folder_id.to_string(),
                ty: String::from("library-playlist-folders"),
            }],
        });
        self
    }

    /// Create the playlist
    ///
    /// # Return type
    ///
    /// Resource of type [`LibraryPlaylist`]
    pub async fn create(mut self, client: &ApiClient) -> Result<Option<LibraryPlaylist>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .post("/v1/me/library/playlists")
            .query(&request_context.query)
            .json(&self.data)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }
}

#[derive(Serialize, Default, Clone, PartialEq, Eq)]
struct LibraryPlaylistAddRequest {
    data: Vec<ThinRelationship>,
}

/// Library playlist folder
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LibraryPlaylistFolder {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Library playlist folders attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<LibraryPlaylistFolderAttributes>,
    /// Library playlist folders relationships
    #[serde(default)]
    pub relationships: LibraryPlaylistFolderRelationships,
}

/// Library folders playlist attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct LibraryPlaylistFolderAttributes {
    /// The date this content added to the user’s library in ISO-8601 format
    #[serde(with = "time::serde::iso8601::option")]
    pub date_added: Option<OffsetDateTime>,
    /// The (potentially) censored name of the content
    pub name: String,
}

/// Library folders relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(
    LibraryPlaylistFolderRelationshipType,
    object = "library-playlist-folders",
    relationship
)]
pub struct LibraryPlaylistFolderRelationships {
    /// The corresponding playlist in the Apple Music catalog the playlist is associated with.
    ///
    /// Fetch limits: None (associated with at most one catalog playlist).
    ///
    /// Possible resources: [`Playlist`]
    pub catalog: Option<Relationship<Playlist>>,
    /// The library songs and library music videos included in the playlist. By default, tracks not included. Only available when fetching a single library playlist resource by ID.
    ///
    /// Fetch limits: 100 default, 100 maximum.
    ///
    /// Possible resources: [`LibraryMusicVideo`], [`LibrarySong`]
    pub tracks: Option<Relationship<Resource>>,
}

/// Library track types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LibraryTrackTypes {
    /// Library music videos
    #[serde(rename = "library-music-videos")]
    MusicVideos,
    /// Library songs
    #[serde(rename = "library-songs")]
    Songs,
}

/// Library playlist folder request builder
pub struct LibraryPlaylistFolderRequestBuilder;

/// Library playlist folder get request builder
pub type LibraryPlaylistFolderGetRequestBuilder<'a> =
    MusicRequestBuilder<'a, LibraryPlaylistFolderRequestBuilder>;

impl<'a> LibraryPlaylistFolderGetRequestBuilder<'a> {
    /// Fetch one library playlist folder by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<LibraryPlaylistFolder>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!("/v1/me/library/playlist-folders/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple library playlist folders by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<LibraryPlaylistFolder>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/me/library/playlist-folders/")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }
}
