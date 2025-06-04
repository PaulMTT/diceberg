use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::core_scope::DicebergClientCoreScoped;
use crate::api::client::iceberg_scope::DicebergClientIcebergScoped;
use crate::api::management::client::DiciManagementClient;
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

#[derive(TypedBuilder, Clone)]
pub struct DicebergClient {
    #[builder(default = default_warehouse(), setter(into))]
    warehouse: String,
}

impl Default for DicebergClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl DicebergClient {
    pub fn core(self, asset: CoreAsset) -> DicebergClientCoreScoped {
        DicebergClientCoreScoped::builder()
            .client(self)
            .asset(asset)
            .build()
    }

    pub fn core_with_client(
        self,
        asset: CoreAsset,
        dici_management_client: DiciManagementClient,
    ) -> DicebergClientCoreScoped {
        DicebergClientCoreScoped::builder()
            .client(self)
            .asset(asset)
            .management_client(dici_management_client)
            .build()
    }

    pub fn iceberg(self, asset: IcebergAsset) -> DicebergClientIcebergScoped {
        DicebergClientIcebergScoped::builder()
            .client(self)
            .asset(asset)
            .build()
    }
}

impl CatalogSource for DicebergClient {
    async fn catalog(&self) -> Result<GlueCatalog> {
        GlueCatalog::new(
            GlueCatalogConfig::builder()
                .warehouse(self.warehouse.clone())
                .build(),
        )
            .await
            .context("Failed to construct glue catalog")
    }
}