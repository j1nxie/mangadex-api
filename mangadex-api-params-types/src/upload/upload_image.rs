use std::path::PathBuf;

#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api::{
    v5::upload::upload_session_id::post::{UploadImage, UploadImagesBuilder},
    MangaDexClient,
};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_schema::{v5::UploadSessionFileDataObject, Limited};
#[cfg(feature = "mangadex-api-resolver")]
use mangadex_api_types::error::Result;

use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
pub struct UploadImageParam {
    pub session_id: Uuid,
    pub files: Vec<PathBuf>,
}

#[cfg(feature = "mangadex-api-resolver")]
impl From<UploadImageParam> for UploadImagesBuilder {
    fn from(value: UploadImageParam) -> Self {
        let mut builder = Self::default();
        builder.session_id(value.session_id);
        let files: Vec<UploadImage> = value
            .files
            .iter()
            .flat_map(<UploadImage as TryFrom<&PathBuf>>::try_from)
            .collect();
        builder.files(files);
        builder
    }
}

#[cfg(feature = "mangadex-api-resolver")]
impl UploadImageParam {
    pub async fn send(
        self,
        client: &MangaDexClient,
    ) -> Result<Limited<UploadSessionFileDataObject>> {
        <UploadImagesBuilder as From<Self>>::from(self)
            .http_client(client.get_http_client().clone())
            .send()
            .await
    }
}
