//! CustomList endpoint handler.
//!
//! <https://api.mangadex.org/swagger.html#/CustomList>

pub mod post;
pub mod id;
use uuid::Uuid;

use crate::v5::custom_list::post::CreateCustomListBuilder;
use crate::HttpClientRef;

/// CustomList endpoint handler builder.
#[derive(Debug)]
pub struct CustomListBuilder {
    http_client: HttpClientRef,
}

impl CustomListBuilder {
    #[doc(hidden)]
    pub(crate) fn new(http_client: HttpClientRef) -> Self {
        Self { http_client }
    }

    /// Create a custom list.
    ///
    /// <https://api.mangadex.org/swagger.html#/CustomList/post-list>
    pub fn post(&self) -> CreateCustomListBuilder {
        CreateCustomListBuilder::default().http_client(self.http_client.clone())
    }

    pub fn id(&self, id : Uuid) {
        todo!()
    }
}
