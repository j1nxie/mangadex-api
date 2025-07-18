//! Builder for finding Chapter statistics.
//!
//! <https://api.mangadex.org/docs/swagger.html#/Statistics/get-statistics-chapters>
//!
//! This allows querying for multiple Chapter.
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api_types::MangaStatus;
//! use mangadex_api::v5::MangaDexClient;
//! use uuid::Uuid;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! // Cool Tsuma no Saa-chan chapter 25
//! let chapter_id = Uuid::parse_str("1c3a8585-8df6-46d1-af98-fa777666abf2")?;
//!
//! let chapter_stats = client
//!     .statistics()
//!     .chapter()
//!     .get()
//!     .chapter_id(chapter_id)
//!     .send()
//!     .await?;
//!
//! println!("Response: {:?}", chapter_stats);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use mangadex_api_schema::v5::statistics::chapter::ChapterStatisticsObject;
use serde::Serialize;
use uuid::Uuid;

use crate::HttpClientRef;

#[cfg_attr(
    feature = "deserializable-endpoint",
    derive(serde::Deserialize, getset::Getters, getset::Setters)
)]
#[derive(Debug, Serialize, Clone, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(
    setter(into, strip_option),
    build_fn(error = "crate::error::BuilderError"),
    default
)]
#[non_exhaustive]
pub struct FindChapterStatistics {
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    #[cfg_attr(feature = "deserializable-endpoint", getset(set = "pub", get = "pub"))]
    pub http_client: HttpClientRef,

    #[builder(setter(each = "chapter_id"))]
    pub chapter: Vec<Uuid>,
}

endpoint! {
    GET "/statistics/chapter",
    // Known issue: Despite the API docs stating that authorization is required, the endpoint is
    // available to guests.
    #[query] FindChapterStatistics,
    #[flatten_result] crate::Result<ChapterStatisticsObject>,
    FindChapterStatisticsBuilder
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use url::Url;
    use uuid::Uuid;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn find_group_statistics_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let manga_id = Uuid::new_v4();

        let thread_id = 4756728;
        let replies_count = 12;

        let response_body = json!({
            "result": "ok",
            "statistics": {
                manga_id.to_string(): {
                    "comments": {
                      "threadId": thread_id,
                      "repliesCount": replies_count
                    }
                }
            }
        });

        Mock::given(method("GET"))
            .and(path("/statistics/chapter"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        let res = mangadex_client
            .statistics()
            .chapter()
            .get()
            .chapter_id(&manga_id)
            .send()
            .await?;
        let ctt = res.statistics.get(&manga_id).ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "This id is not found",
        ))?;
        let comments = ctt.comments.ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "The comment is not found",
        ))?;
        assert_eq!(comments.thread_id, thread_id);
        assert_eq!(comments.replies_count, replies_count);
        Ok(())
    }
}
