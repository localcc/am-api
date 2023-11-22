//! Apple music primitive types

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Play parameters
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayParameters {
    /// Song id
    pub id: String,
    /// Parameters kind
    pub kind: String,
}

/// Editorial notes
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct EditorialNotes {
    /// Name for the editorial notes
    pub name: Option<String>,
    /// Abbreviated notes that display inline or when the content appears alongside other content
    pub short: Option<String>,
    /// Notes that appear when the content displays prominently
    pub standard: Option<String>,
    /// Tag line for the editorial notes
    pub tagline: Option<String>,
}

/// Preview
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct Preview {
    /// Preview url
    pub url: String,
}

/// Audio variants
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudioVariant {
    /// Dolby atmos
    #[serde(rename = "dolby-atmos")]
    DolbyAtmos,
    /// Dolby audio
    #[serde(rename = "dolby-audio")]
    DolbyAudio,
    /// Hi-Res lossless
    #[serde(rename = "hi-res-lossless")]
    HiResLossless,
    /// Lossless
    #[serde(rename = "lossless")]
    Lossless,
    /// Lossy stereo
    #[serde(rename = "lossy-stereo")]
    LossyStereo,
}

/// Content rating
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentRating {
    /// Clean
    #[serde(rename = "clean")]
    Clean,
    /// Explicit
    #[serde(rename = "explicit")]
    Explicit,
}

/// Track types
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TrackType {
    /// Library music video
    ///
    /// Resource: [`LibraryMusicVideo`]
    #[serde(rename = "library-music-videos")]
    LibraryMusicVideo,
    /// Library song
    ///
    /// Resource: [`LibrarySong`]
    #[serde(rename = "library-songs")]
    LibrarySong,
    /// Music video
    ///
    /// Resource: [`MusicVideo`]
    #[serde(rename = "music-videos")]
    MusicVideo,
    /// Song
    ///
    /// Resource: [`Song`]
    #[serde(rename = "songs")]
    Song,
}

impl Display for TrackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            TrackType::LibraryMusicVideo => "library-music-videos",
            TrackType::LibrarySong => "library-songs",
            TrackType::MusicVideo => "music-videos",
            TrackType::Song => "songs",
        };
        write!(f, "{}", string)
    }
}
