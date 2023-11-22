//! Request builders and structures

use crate::error::Error;
use crate::resource::{ErrorResponse, ResourceResponse};
use reqwest::Response;
use serde::de::DeserializeOwned;

pub mod builder;
pub(crate) mod context;
pub mod extension;
pub(crate) mod paginated;
pub mod relationship;
pub mod view;

/// Default fetch entries limit for a page
pub const DEFAULT_FETCH_LIMIT: usize = 21;

pub(crate) async fn try_resource_response<R>(
    response: Response,
) -> Result<ResourceResponse<R>, Error>
where
    R: DeserializeOwned,
{
    if !response.status().is_success() {
        let error_response: ErrorResponse = response.json().await?;
        return Err(Error::MusicError(error_response));
    }

    Ok(response.json().await?)
}
