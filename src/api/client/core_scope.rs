use crate::api::client::asset::CoreAsset;
use crate::api::client::base::DicebergClient;
use crate::api::management::client::DiciManagementClient;
use crate::api::traits::{ClientSource, TableIdentitySource, TableReferenceSource};
use anyhow::{Context, Result};
use datafusion::common::TableReference;
use iceberg::TableIdent;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone)]
pub struct DicebergCoreAsset {
    client: DicebergClient,
    asset: CoreAsset,
    #[builder(default)]
    management_client: DiciManagementClient,
}

impl TableIdentitySource for DicebergCoreAsset {
    async fn table_ident(&self) -> Result<TableIdent> {
        let inventory = self
            .management_client
            .fetch_inventory_by_fxf(self.asset.fxf.clone())
            .await
            .context("Could not fetch inventory")?;

        TableIdent::from_strs([
            inventory.id.iceberg_location.iceberg_location,
            inventory.id.schema_table.schema_table,
        ])
            .context("Failed to parse table ident from core asset")
    }
}

impl ClientSource for DicebergCoreAsset {
    fn client(&self) -> &DicebergClient {
        &self.client
    }
}

impl TableReferenceSource for DicebergCoreAsset {
    async fn table_reference(&self) -> Result<TableReference> {
        Ok(TableReference::Bare {
            table: self.asset.fxf.clone().into(),
        })
    }
}
