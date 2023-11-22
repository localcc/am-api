//! Apple music api
#![deny(missing_docs)]

use crate::error::Error;
pub use celes;
use reqwest::{header, RequestBuilder};

pub mod error;
pub mod primitive;
pub mod request;
pub mod resource;
pub mod time;

/// Cast a Resource to a more specific type
///
/// # Examples
///
/// ```no_run,ignore
/// use
/// let a: Resource = ...;
/// let b: &DoubleProperty = cast!(Property, DoubleProperty, &a).unwrap();
/// ```
#[macro_export]
macro_rules! cast {
    ($type:path, $field:expr) => {
        match $field {
            $type { data } => Some(data),
            _ => None,
        }
    };
}

/// Apple music api client
///
/// Api client can be cloned safely as reqwest uses Arc internally
#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    storefront_country: celes::Country,
    localization: String,
}

impl ApiClient {
    /// Create a new [`ApiClient`] instance
    pub fn new(
        developer_token: &str,
        media_user_token: &str,
        storefront_country: celes::Country,
    ) -> Result<ApiClient, Error> {
        let mut headers = header::HeaderMap::new();

        let mut authorization_header =
            header::HeaderValue::from_str(&format!("Bearer {}", developer_token))?;
        authorization_header.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, authorization_header);

        let mut media_user_token_header = header::HeaderValue::from_str(media_user_token)?;
        media_user_token_header.set_sensitive(true);
        headers.insert("media-user-token", media_user_token_header);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(ApiClient {
            client,
            storefront_country,
            localization: String::from("en-US"),
        })
    }

    /// Get the default storefront country for this client
    pub fn get_storefront_country(&self) -> celes::Country {
        self.storefront_country
    }

    /// Get the default localization for this client
    pub fn get_localization(&self) -> &str {
        self.localization.as_str()
    }

    /// Set the default localization for this client
    pub fn set_localization(&mut self, localization: &str) {
        self.localization = localization.to_string();
    }

    /// Convenience method to make a GET request to an endpoint
    pub fn get(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .get(format!("https://api.music.apple.com{}", endpoint))
            .query(&[("art[url]", "f")])
    }

    /// Convenience method to make a POST request to an endpoint
    pub fn post(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .post(format!("https://api.music.apple.com{}", endpoint))
            .query(&[("art[url]", "f")])
    }

    /// Convenience method to make a PUT request to an endpoint
    pub fn put(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .put(format!("https://api.music.apple.com{}", endpoint))
            .query(&[("art[url]", "f")])
    }

    /// Convenience method to make a DELETE request to an endpoint
    pub fn delete(&self, endpoint: &str) -> RequestBuilder {
        self.client
            .delete(format!("https://api.music.apple.com{}", endpoint))
            .query(&[("art[url]", "f")])
    }
}
