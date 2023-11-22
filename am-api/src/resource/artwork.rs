//! Artwork information

use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_hex::{Compact, SerHex};
use tinytemplate::TinyTemplate;

/// Artwork information
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Artwork {
    /// Original image width in pixels
    pub width: u32,
    /// Original image height in pixels
    pub height: u32,
    /// Template image url
    ///
    /// DO NOT USE FOR REQUESTS:
    /// for getting the image use the [`Artwork::get_image`] method
    pub url: String,
    /// Text color 1 in rgb hex
    #[serde(default)]
    pub text_color_1: Option<HexColor>,
    /// Text color 2 in rgb hex
    #[serde(default)]
    pub text_color_2: Option<HexColor>,
    /// Text color 3 in rgb hex
    #[serde(default)]
    pub text_color_3: Option<HexColor>,
    /// Text color 4 in rgb hex
    #[serde(default)]
    pub text_color_4: Option<HexColor>,
}

/// Hex color
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexColor(#[serde(with = "SerHex::<Compact>")] u32);

impl AsRef<u32> for HexColor {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl From<HexColor> for u32 {
    fn from(val: HexColor) -> Self {
        val.0
    }
}

impl From<u32> for HexColor {
    fn from(value: u32) -> Self {
        HexColor(value)
    }
}

/// Artwork image formats
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArtworkImageFormat {
    /// Png image format,
    Png,
    /// Webp image format
    Webp,
    /// Jpeg image format
    Jpeg,
}

impl ArtworkImageFormat {
    fn get_format_string(&self) -> &str {
        match self {
            ArtworkImageFormat::Png => "png",
            ArtworkImageFormat::Webp => "webp",
            ArtworkImageFormat::Jpeg => "jpg",
        }
    }
}

impl Artwork {
    /// Get artwork image url
    ///
    /// # Parameters
    ///
    /// * width - preferred width
    ///
    /// * height - preferred height
    ///
    /// * image_format - image format in which the image should be retrieved
    pub fn get_image_url(
        &self,
        width: u32,
        height: u32,
        image_format: ArtworkImageFormat,
    ) -> Result<String, Error> {
        let mut tt = TinyTemplate::new();
        tt.add_template("url", &self.url)?;

        #[derive(Serialize)]
        struct UrlContext<'a> {
            w: u32,
            h: u32,
            f: &'a str,
        }

        let context = UrlContext {
            w: width,
            h: height,
            f: image_format.get_format_string(),
        };

        Ok(tt.render("url", &context)?)
    }
}
