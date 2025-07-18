#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{v5::user::id::list::get::UserCustomListsBuilder, MangaDexClient};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::InputObject))]
pub struct UserCustomListParams {
    pub user_id: Uuid,

    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    limit: Option<u32>,
    #[serde(default)]
    #[cfg_attr(feature = "async-graphql", graphql(default))]
    offset: Option<u32>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UserCustomListParams> for UserCustomListsBuilder {
    fn from(value: UserCustomListParams) -> Self {
        let mut builder = Self::default();
        builder.user_id(value.user_id);
        if let Some(limit) = value.limit {
            builder.limit(limit);
        }
        if let Some(offset) = value.offset {
            builder.offset(offset);
        }
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UserCustomListParams {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> mangadex_api::Result<mangadex_api_schema::v5::CustomListCollection> {
        <UserCustomListsBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
