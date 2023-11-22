//! Library artist

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::catalog::artist::Artist;
use crate::resource::library::album::LibraryAlbum;
use crate::resource::relationship::Relationship;
use crate::resource::{ResourceHeader};
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Library artist
#[derive(Context, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LibraryArtist {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Library artist attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<LibraryArtistAttributes>,
    /// Library artist relationships
    #[serde(default)]
    pub relationships: LibraryArtistRelationships,
}

/// Library artist attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct LibraryArtistAttributes {
    /// The artist's name
    pub name: String,
}

/// Library artist relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(
    LibraryArtistRelationshipType,
    object = "library-artists",
    relationship
)]
pub struct LibraryArtistRelationships {
    /// The library albums associated with the artist. By default, albums not included. It’s available only when fetching a single library artist resource by ID.
    ///
    /// Fetch limits: 25 default, 100 maximum
    ///
    /// Possible resources: [`LibraryAlbum`]
    pub albums: Option<Relationship<LibraryAlbum>>,
    /// The artist in the Apple Music catalog the library artist is associated with, when known.
    ///
    /// Fetch limits: None (associated with, at most, one catalog artist).
    ///
    /// Possible resources: [`Artist`]
    pub catalog: Option<Relationship<Artist>>,
}

/// Library artist request builder
pub struct LibraryArtistRequestBuilder;

/// Library artist get request builder
pub type LibraryArtistGetRequestBuilder<'a> = MusicRequestBuilder<'a, LibraryArtistRequestBuilder>;

impl<'a> LibraryArtistGetRequestBuilder<'a> {
    /// Fetch one library artist by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<LibraryArtist>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!("/v1/me/library/artists/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple library artists by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<LibraryArtist>, Error> {
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

    /// Fetch all library artists
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
    ) -> impl Stream<Item = Result<LibraryArtist, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/library/artists"),
            request_context,
            offset,
        )
    }
}
