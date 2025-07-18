//! User endpoint handler.
//!
//! <https://api.mangadex.org/docs/swagger.html#/User>

cfg_custom_list_v2! {
    pub mod bookmarks;
}

pub mod follows;
pub mod get;
pub mod history;
pub mod id;
pub mod list;
pub mod me;
cfg_custom_list_v2! {
    pub mod subscription;
}

use crate::HttpClientRef;
use uuid::Uuid;

#[cfg(feature = "custom_list_v2")]
use bookmarks::BookmarksEndpoint;
use follows::FollowsEndpoint;
use get::ListUserBuilder;
use history::HistoryEndpoint;
use id::IdEndpoint;
use list::ListEndpoint;
use me::MeEndpoint;
#[cfg(feature = "custom_list_v2")]
use subscription::SubscriptionEndpoint;

#[derive(Debug)]
pub struct UserBuilder {
    http_client: HttpClientRef,
}

impl UserBuilder {
    pub fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }
    cfg_custom_list_v2! {
        pub fn bookmarks(&self) -> BookmarksEndpoint {
            BookmarksEndpoint::new(self.http_client.clone())
        }
    }

    pub fn get(&self) -> ListUserBuilder {
        ListUserBuilder::default().http_client(self.http_client.clone())
    }

    pub fn follows(&self) -> FollowsEndpoint {
        FollowsEndpoint::new(self.http_client.clone())
    }

    pub fn history(&self) -> HistoryEndpoint {
        HistoryEndpoint::new(self.http_client.clone())
    }

    pub fn id(&self, id: Uuid) -> IdEndpoint {
        IdEndpoint::new(self.http_client.clone(), id)
    }

    pub fn list(&self) -> ListEndpoint {
        ListEndpoint::new(self.http_client.clone())
    }

    pub fn me(&self) -> MeEndpoint {
        MeEndpoint::new(self.http_client.clone())
    }

    cfg_custom_list_v2! {
        pub fn subscription(&self) -> SubscriptionEndpoint {
            SubscriptionEndpoint::new(self.http_client.clone())
        }
    }
}
