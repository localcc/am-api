//! Personal recommendation

use crate::error::Error;
use crate::request::builder::MusicRequestBuilder;
use crate::request::context::ContextContainer;
use crate::request::paginated::paginate;
use crate::request::try_resource_response;
use crate::resource::relationship::Relationship;
use crate::resource::{Resource, ResourceHeader};
use crate::ApiClient;
use am_api_proc_macro::{Context, ResourceProperty};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;

/// Personal recommendation
#[derive(Context, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PersonalRecommendation {
    /// Resource header
    #[context(skip)]
    #[serde(flatten)]
    pub header: ResourceHeader,
    /// Personal recommendation attributes
    #[context(skip)]
    #[serde(default)]
    pub attributes: Option<PersonalRecommendationAttributes>,
    /// Personal recommendation relationships
    #[serde(default)]
    pub relationships: PersonalRecommendationRelationships,
}

impl PersonalRecommendation {
    /// Get personal recommendation request builder
    pub fn get<'a>() -> PersonalRecommendationGetRequestBuilder<'a> {
        PersonalRecommendationGetRequestBuilder::default()
    }
}

/// Personal recommendation attributes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PersonalRecommendationAttributes {
    /// Recommendation kind
    pub kind: PersonalRecommendationKind,
    /// The next date in UTC format for updating the recommendation
    #[serde(with = "time::serde::iso8601")]
    pub next_update_date: OffsetDateTime,
    /// The localized reason for the recommendation
    #[serde(default)]
    pub reason: Option<PersonalRecommendationReason>,
    /// The resource types supported by the recommendation
    pub resource_types: Vec<String>,
    /// The localized title for the recommendation
    #[serde(default)]
    pub title: Option<PersonalRecommendationTitle>,
}

/// Personal recommendation relationships
#[derive(
    Context, ResourceProperty, Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase", default)]
#[resource_property(
    PersonalRecommendationRelationshipType,
    object = "recommendations",
    relationship
)]
pub struct PersonalRecommendationRelationships {
    /// The contents associated with the content recommendation type. By default, contents includes objects.
    ///
    /// Fetch limits: 10 default, 10 maximum.
    ///
    /// Possible resources: all
    pub contents: Option<Relationship<Resource>>,
}

/// Personal recommendation kind
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PersonalRecommendationKind {
    /// A recommendation for music content
    #[serde(rename = "music-recommendations")]
    #[default]
    MusicRecommendations,
    /// A recommendation based on recently played content
    #[serde(rename = "recently-played")]
    RecentlyPlayed,
    /// A generic recommendation type
    #[serde(rename = "unknown")]
    Generic,
}

/// Personal recommendation reason
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct PersonalRecommendationReason {
    /// The localized reason for the recommendation
    pub string_for_display: String,
}

/// Personal recommendation title
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", default)]
pub struct PersonalRecommendationTitle {
    /// The localized title for the recommendation
    pub string_for_display: String,
}

/// Personal recommendation request builder
pub struct PersonalRecommendationRequestBuilder;

/// Personal recommendation get request builder
pub type PersonalRecommendationGetRequestBuilder<'a> =
    MusicRequestBuilder<'a, PersonalRecommendationRequestBuilder>;

impl<'a> PersonalRecommendationGetRequestBuilder<'a> {
    /// Fetch one recommendation by id
    pub async fn one(
        mut self,
        client: &ApiClient,
        id: &str,
    ) -> Result<Option<PersonalRecommendation>, Error> {
        let request_context = Arc::new(self.get_request_context_drain(client));

        let response = client
            .get(&format!("/v1/me/recommendations/{id}"))
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data.into_iter().next())
    }

    /// Fetch multiple recommendations by id
    pub async fn many(
        mut self,
        client: &ApiClient,
        ids: &[&str],
    ) -> Result<Vec<PersonalRecommendation>, Error> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("ids"), ids.to_vec().join(",")));
        let request_context = Arc::new(request_context);

        let response = client
            .get("/v1/me/recommendations")
            .query(&request_context.query)
            .send()
            .await?;

        let mut response = try_resource_response(response).await?;
        response.data.set_context(request_context);
        Ok(response.data)
    }

    /// Fetch default recommendations
    ///
    /// # Params
    ///
    /// * limit - limit of entries per query
    ///
    /// * offset - query offset
    pub async fn default_recommendations(
        mut self,
        client: &ApiClient,
        limit: usize,
        offset: usize,
    ) -> impl Stream<Item = Result<PersonalRecommendation, Error>> {
        let mut request_context = self.get_request_context_drain(client);
        request_context
            .query
            .push((String::from("limit"), limit.to_string()));

        paginate(
            client.clone(),
            String::from("/v1/me/recommendations"),
            request_context,
            offset,
        )
    }
}
