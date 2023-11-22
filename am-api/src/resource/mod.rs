//! Apple music resources
use am_api_proc_macro::Context;

use serde::{Deserialize, Serialize};

pub mod artwork;
pub mod attributes;
pub mod catalog;
pub mod genre;
pub mod history;
pub mod library;
pub mod personal_recommendation;
pub mod rating;
pub mod relationship;
pub mod storefront;
pub mod view;

/// Apple music resource header
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct ResourceHeader {
    /// Identifier
    pub id: String,
    /// Relative location for the resource
    pub href: String,
}

/// Trait for getting resource information
pub trait ResourceInfo {
    /// Get header
    fn get_header(&self) -> &ResourceHeader;
}

/// Trait for getting resource data type
pub(crate) trait ResourceType {
    /// Get resource type
    fn get_type(&self) -> &'static str;
}

macro_rules! resource {
    ($($name:literal => $enum_name:ident : $data_type:path),*) => {
        /// Apple music resource data
        #[derive(Context, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
        #[serde(tag = "type")]
        pub enum Resource {
            $(
                #[doc = $name]
                #[serde(rename = $name)]
                $enum_name {
                    /// Data
                    #[serde(flatten)]
                    data: $data_type
                }
            ),*
        }

        impl ResourceInfo for Resource {
            fn get_header(&self) -> &ResourceHeader {
                match self {
                    $(Self::$enum_name { data } => &data.header),*
                }
            }
        }

        impl ResourceType for Resource {
            fn get_type(&self) -> &'static str {
                match self {
                    $(Self::$enum_name { .. } => $name),*
                }
            }
        }

        $(
            impl From<$data_type> for Resource {
                fn from(data: $data_type) -> Self {
                    Self::$enum_name { data }
                }
            }
        )*
    }
}

resource! {
    "activities" => Activity : catalog::activity::Activity,
    "albums" => Album : catalog::album::Album,
    "artists" => Artist : catalog::artist::Artist,
    "apple-curators" => AppleCurator : catalog::curator::AppleCurator,
    "curators" => Curator : catalog::curator::Curator,
    "genres" => Genre : genre::Genre,
    "music-videos" => MusicVideo : catalog::music_video::MusicVideo,
    "personal-recommendation" => PersonalRecommendation : personal_recommendation::PersonalRecommendation,
    "playlists" => Playlist : catalog::playlist::Playlist,
    "ratings" => Rating : rating::Rating,
    "record-labels" => RecordLabel : catalog::record_label::RecordLabel,
    "songs" => Song : catalog::song::Song,
    "stations" => Station : catalog::station::Station,
    "station-genres" => StationGenre : catalog::station::StationGenre,
    "library-albums" => LibraryAlbum : library::album::LibraryAlbum,
    "library-artists" => LibraryArtist : library::artist::LibraryArtist,
    "library-music-videos" => LibraryMusicVideo : library::music_video::LibraryMusicVideo,
    "library-playlists" => LibraryPlaylist : library::playlist::LibraryPlaylist,
    "library-playlist-folders" => LibraryPlaylistFolder : library::playlist::LibraryPlaylistFolder,
    "library-songs" => LibrarySong : library::song::LibrarySong
}

/// Apple music response
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResponse<R = Resource> {
    /// Data
    pub data: Vec<R>,
}

/// Apple music error response
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// Error code
    #[serde(default)]
    pub code: Option<i32>,
    /// Error message
    #[serde(default)]
    pub message: Option<String>,
    /// Errors
    #[serde(default)]
    pub errors: Vec<MusicError>,
}

/// Apple music error
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct MusicError {
    /// Error id
    pub id: String,
    /// Error title
    pub title: String,
    /// Error detail
    pub detail: String,
    /// Status
    pub status: String,
    /// Error code
    pub code: String,
}
