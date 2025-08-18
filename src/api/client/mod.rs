pub mod asset;
use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::management::client::ManagementClient;
use crate::api::traits::CatalogSource;
use anyhow::{Context, Result};
use iceberg_catalog_glue::{GlueCatalog, GlueCatalogConfig};
use std::env;
use typed_builder::TypedBuilder;
fn warehouse_from_env() -> Result<String> {
    env::var("DICI_WAREHOUSE").context("DICI_WAREHOUSE is not set")
}
fn default_warehouse() -> String {
    warehouse_from_env()
        .context("Could not determine warehouse")
        .unwrap()
}
pub enum DiciAsset {
    Core {
        asset: CoreAsset,
        dici_client: DiciClient,
        management_client: ManagementClient,
    },
    Iceberg {
        asset: IcebergAsset,
        client: DiciClient,
    },
}
pub type Warehouse = String;
#[derive(TypedBuilder, Clone)]
pub struct DiciConfig {
    #[builder(default = default_warehouse(), setter(into))]
    warehouse: Warehouse,
}
impl Default for DiciConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
#[derive(TypedBuilder, Clone)]
pub struct DiciClient {
    #[builder(default)]
    config: DiciConfig,
}
impl Default for DiciClient {
    fn default() -> Self {
        Self::builder().build()
    }
}
impl CatalogSource for DiciClient {
    async fn catalog(&self) -> Result<GlueCatalog> {
        GlueCatalog::new(
            GlueCatalogConfig::builder()
                .warehouse(self.config.warehouse.clone())
                .build(),
        )
        .await
        .context("Failed to construct glue catalog")
    }
}
