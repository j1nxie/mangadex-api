//! Builder for the CustomList creation endpoint.
//!
//! <https://api.mangadex.org/swagger.html#/CustomList/post-list>
//!
//! # Examples
//!
//! ```rust
//! use uuid::Uuid;
//!
//! use mangadex_api::v5::MangaDexClient;
//! use mangadex_api_types::{Password, Username};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let _login_res = client
//!     .auth()
//!     .login()
//!     .username(Username::parse("myusername")?)
//!     .password(Password::parse("hunter23")?)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! let manga_id = Uuid::new_v4();
//! let res = client
//!     .custom_list()
//!     .create()
//!     .name("My Custom List")
//!     .add_manga_id(manga_id)
//!     .version(1_u32)
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("custom list create: {:?}", res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;
use mangadex_api_schema::v5::CustomListResponse;
use mangadex_api_types::CustomListVisibility;

#[derive(Debug, Deserialize, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option), pattern = "owned")]
#[non_exhaustive]
pub struct CreateCustomList<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub visibility: Option<CustomListVisibility>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(setter(each = "add_manga_id"), default)]
    pub manga: Vec<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub version: Option<u32>,
}

endpoint! {
    POST ("/list"),
    #[body auth] CreateCustomList<'_>,
    #[flatten_result] CustomListResponse
}

#[cfg(test)]
mod tests {
    use fake::faker::name::en::Name;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::v5::AuthTokens;
    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn create_custom_list_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .auth_tokens(AuthTokens {
                session: "sessiontoken".to_string(),
                refresh: "refreshtoken".to_string(),
            })
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let custom_list_id = Uuid::new_v4();
        let custom_list_name: String = Name().fake();
        let _expected_body = json!({
            "name": custom_list_name,
            "version": 1
        });
        let response_body = json!({
            "result": "ok",
            "response": "entity",
            "data": {
                "id": custom_list_id,
                "type": "custom_list",
                "attributes": {
                    "name": custom_list_name,
                    "visibility": "private",
                    "version": 1
                },
                "relationships": []
            }
        });

        Mock::given(method("POST"))
            .and(path(r"/list"))
            .and(header("Authorization", "Bearer sessiontoken"))
            .and(header("Content-Type", "application/json"))
            // TODO: Make the request body check work.
            // .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .custom_list()
            .create()
            .name(custom_list_name.as_str())
            .version(1u32)
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
