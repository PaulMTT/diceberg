use crate::api::http::management::model::inventory::Inventory;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IcebergAssetImpl {
    pub iceberg_location: String,
    pub schema_table: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DomainIcebergAssetImpl {
    pub domain: String,
    pub iceberg_location: String,
    pub schema_table: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IcebergLocationSync {
    pub successes: Vec<Inventory>,
    pub failures: Vec<Failed>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Failed {
    pub inventory: Inventory,
    pub reason: String,
}
