//! General CustomList information.

use mangadex_api_types::CustomListVisibility;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CustomListAttributes {
    pub name: String,
    pub visibility: CustomListVisibility,
    pub version: u32,
}
