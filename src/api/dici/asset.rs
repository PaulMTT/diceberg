use crate::api::dici::catalog::DiciCatalog;
use crate::api::dici::core::CoreAsset;
use crate::api::dici::iceberg::IcebergAsset;
use crate::api::management::client::ManagementClient;
use crate::api::traits::{CatalogSource, TableIdentitySource, TableReferenceSource};
use anyhow::{Context, Result};
use datafusion::common::TableReference;
use iceberg::TableIdent;
use iceberg_catalog_glue::GlueCatalog;
pub enum DiciAsset {
    Core {
        asset: CoreAsset,
        dici_catalog: DiciCatalog,
        management_client: ManagementClient,
    },
    Iceberg {
        asset: IcebergAsset,
        dici_catalog: DiciCatalog,
    },
}
impl CatalogSource for DiciAsset {
    fn catalog(&self) -> impl Future<Output = Result<GlueCatalog>> {
        let dici_catalog = match self {
            DiciAsset::Core { dici_catalog, .. } => dici_catalog,
            DiciAsset::Iceberg { dici_catalog, .. } => dici_catalog,
        };
        dici_catalog.catalog()
    }
}
impl TableReferenceSource for DiciAsset {
    async fn table_reference(&self) -> Result<TableReference> {
        match self {
            DiciAsset::Core { asset, .. } => asset.table_reference().await,
            DiciAsset::Iceberg { asset, .. } => asset.table_reference().await,
        }
    }
}
impl TableIdentitySource for DiciAsset {
    async fn table_ident(&self) -> Result<TableIdent> {
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
