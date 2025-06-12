pub mod asset;
use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::management::client::ManagementClient;
use crate::api::traits::{CatalogSource, ClientSource, TableIdentitySource, TableReferenceSource};
use anyhow::Context;
use datafusion::common::TableReference;
use iceberg::TableIdent;
use iceberg_catalog_glue::{GlueCatalog, GlueCatalogConfig};
use std::env;
use typed_builder::TypedBuilder;

fn warehouse_from_env() -> anyhow::Result<String> {
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
    async fn catalog(&self) -> anyhow::Result<GlueCatalog> {
        GlueCatalog::new(
            GlueCatalogConfig::builder()
                .warehouse(self.config.warehouse.clone())
                .build(),
        )
        .await
        .context("Failed to construct glue catalog")
    }
}

impl ClientSource for DiciAsset {
    fn client(&self) -> &DiciClient {
        match self {
            DiciAsset::Core {
                dici_client: client,
                ..
            } => client,
            DiciAsset::Iceberg { client, .. } => client,
        }
    }
}

impl TableReferenceSource for DiciAsset {
    async fn table_reference(&self) -> anyhow::Result<TableReference> {
        Ok(match self {
            DiciAsset::Core {
                asset: CoreAsset { fxf },
                ..
            } => TableReference::Bare {
                table: fxf.as_str().into(),
            },
            DiciAsset::Iceberg {
                asset: IcebergAsset { schema_table, .. },
                ..
            } => TableReference::Bare {
                table: schema_table.as_str().into(),
            },
        })
    }
}

impl TableIdentitySource for DiciAsset {
    async fn table_ident(&self) -> anyhow::Result<TableIdent> {
        match self {
            DiciAsset::Core {
                asset: CoreAsset { fxf },
                management_client,
                ..
            } => {
                let inventory = management_client
                    .fetch_inventory_by_fxf(fxf.into())
                    .await
                    .context("Could not fetch inventory")?;

                TableIdent::from_strs([
                    inventory.id.iceberg_location.iceberg_location,
                    inventory.id.schema_table.schema_table,
                ])
                .context("Failed to parse table ident from core asset")
            }
            DiciAsset::Iceberg {
                asset:
                    IcebergAsset {
                        location,
                        schema_table,
                    },
                ..
            } => TableIdent::from_strs([location, schema_table])
                .context("Failed to parse table ident from iceberg asset"),
        }
    }
}
