//! Error types

use crate::resource::ErrorResponse;
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

/// Error type
#[derive(Error, Debug)]
pub enum Error {
    /// Missing resource data
    #[error("Missing resource data on a resource")]
    MissingResourceData,
    /// Invalid resource type error
    #[error("Invalid resource type")]
    InvalidResourceType,
    /// Apple music error
    #[error("Apple music error: {0:#?}")]
    MusicError(ErrorResponse),
    /// Invalid header value
    #[error("Invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    /// A [`reqwest::Error`] occured
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// A [`tinytemplate::error::Error`] occurred
    #[error(transparent)]
    TinyTemplate(#[from] tinytemplate::error::Error),
    /// A [`serde_json::Error`] occured
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
